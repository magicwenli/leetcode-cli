//! Edit command
use super::Command;
use crate::config::consts::*;
use crate::{Error, Result};
use anyhow::anyhow;
use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command as ClapCommand};
use std::collections::HashMap;

/// Abstract `edit` command
///
/// ```sh
/// leetcode-edit
/// Edit question by id
///
/// USAGE:
///     leetcode edit <id>
///
/// FLAGS:
///     -h, --help       Prints help information
///     -V, --version    Prints version information
///
/// ARGS:
///     <id>    question id
/// ```

pub struct EditCommand;

#[async_trait]
impl Command for EditCommand {
    /// `edit` usage
    fn usage() -> ClapCommand {
        ClapCommand::new("edit")
            .about("Edit question by id")
            .visible_alias("e")
            .arg(
                Arg::new("lang")
                    .short('l')
                    .long("lang")
                    .num_args(1)
                    .help("Edit with specific language"),
            )
            .arg(
                Arg::new("id")
                    .num_args(1)
                    .required(true)
                    .value_parser(clap::value_parser!(i32))
                    .help("question id"),
            )
    }

    /// `edit` handler
    async fn handler(m: &ArgMatches) -> Result<()> {
        use crate::{cache::models::Question, Cache};
        use std::fs::File;
        use std::io::Write;
        use std::path::Path;

        let id = *m.get_one::<i32>("id").ok_or(Error::NoneError)?;
        let cache = Cache::new()?;
        let problem = cache.get_problem(id)?;
        let mut conf = cache.to_owned().0.conf;

        let test_flag = conf.code.test;

        // condition language
        if m.contains_id("lang") {
            conf.code.lang = m
                .get_one::<String>("lang")
                .ok_or(Error::NoneError)?
                .to_string();
            conf.sync()?;
        }

        let lang = &conf.code.lang;
        let path = crate::helper::code_path(&problem, Some(lang.to_owned()))?;

        if !Path::new(&path).exists() {
            let mut qr = serde_json::from_str(&problem.desc);
            if qr.is_err() {
                qr = Ok(cache.get_question(id).await?);
            }

            let question: Question = qr?;

            let mut file_code = File::create(&path)?;
            let question_desc = question.desc_comment(&conf);

            let test_path = crate::helper::test_cases_path(&problem)?;

            let mut flag = false;
            for d in question.defs.0 {
                if d.value == *lang {
                    flag = true;

                    let source = include_str!("../code.tmpl")
                        .replace(PROBLEM_TITLE, problem.name.as_str())
                        .replace(
                            PROBLEM_TITLE_SLUG,
                            problem
                                .name
                                .to_string()
                                .to_lowercase()
                                .replace(" ", "_")
                                .as_str(),
                        )
                        .replace(PROBLEM_ID, problem.fid.to_string().as_str())
                        .replace(PROBLEM_DESC, question_desc.as_str())
                        .replace(
                            PROBLEM_LINK,
                            conf.sys
                                .urls
                                .problem
                                .replace("$slug", &problem.slug)
                                .as_str(),
                        )
                        .replace(
                            DISCUSS_LINK,
                            conf.sys
                                .urls
                                .discuss
                                .replace("$slug", &problem.slug)
                                .as_str(),
                        )
                        .replace(PROBLEM_DEFAULT_CODE, d.code.to_string().as_str())
                        .replace(PROBLEM_LEVEL, problem.display_level())
                        .replace(PROBLEM_PERCENT, format!("{}%", problem.percent).as_str())
                        .replace(PROBLEM_CATEGORY, problem.category.as_str())
                        .replace(COMMENT_PREFIX, conf.code.comment_prefix.as_str())
                        .replace(COMMENT_LEADING, conf.code.comment_leading.as_str())
                        .replace(COMMENT_SUFFIX, conf.code.comment_suffix.as_str())
                        .replace(CODE_START_MARKER, conf.code.start_marker.as_str())
                        .replace(CODE_END_MARKER, conf.code.end_marker.as_str());
                    file_code.write_all(source.as_bytes())?;
                }
            }

            // if language is not found in the list of supported languges clean up files
            if !flag {
                std::fs::remove_file(&path)?;

                if test_flag {
                    std::fs::remove_file(&test_path)?;
                }

                return Err(
                    anyhow!("This question doesn't support {lang}, please try another").into(),
                );
            }
        }

        // Get arguments of the editor
        //
        // for example:
        //
        // ```toml
        // [code]
        // editor = "emacsclient"
        // editor_args = [ "-n", "-s", "doom" ]
        // ```
        //
        // ```rust
        // Command::new("emacsclient").args(&[ "-n", "-s", "doom", "<problem>" ])
        // ```
        let mut args: Vec<String> = Default::default();
        if let Some(editor_args) = conf.code.editor_args {
            args.extend_from_slice(&editor_args);
        }

        // Set environment variables for editor
        //
        // for example:
        //
        // ```toml
        // [code]
        // editor = "nvim"
        // editor_envs = [ "XDG_DATA_HOME=...", "XDG_CONFIG_HOME=...", "XDG_STATE_HOME=..." ]
        // ```
        //
        // ```rust
        // Command::new("nvim").envs(&[ ("XDG_DATA_HOME", "..."), ("XDG_CONFIG_HOME", "..."), ("XDG_STATE_HOME", "..."), ]);
        // ```
        let mut envs: HashMap<String, String> = Default::default();
        if let Some(editor_envs) = &conf.code.editor_envs {
            for env in editor_envs.iter() {
                let parts: Vec<&str> = env.split('=').collect();
                if parts.len() == 2 {
                    let name = parts[0].trim();
                    let value = parts[1].trim();
                    envs.insert(name.to_string(), value.to_string());
                } else {
                    return Err(anyhow!("Invalid editor environment variable: {env}").into());
                }
            }
        }

        args.push(path);
        std::process::Command::new(conf.code.editor)
            .envs(envs)
            .args(args)
            .status()?;
        Ok(())
    }
}
