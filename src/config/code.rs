//! Code in config
use serde::{Deserialize, Serialize};

pub mod consts {
    pub const PROBLEM_TITLE: &str = "__PROBLEM_TITLE__";
    pub const PROBLEM_TITLE_SLUG: &str = "__PROBLEM_TITLE_SLUG__";
    pub const PROBLEM_ID: &str = "__PROBLEM_ID__";
    pub const PROBLEM_DESC: &str = "__PROBLEM_DESC__";
    pub const PROBLEM_DEFAULT_CODE: &str = "__PROBLEM_DEFAULT_CODE__";
    pub const PROBLEM_LINK: &str = "__PROBLEM_LINK__";
    pub const PROBLEM_LEVEL: &str = "__PROBLEM_LEVEL__";
    pub const PROBLEM_PERCENT: &str = "__PROBLEM_PERCENT__";
    pub const PROBLEM_CATEGORY: &str = "__PROBLEM_CATEGORY__";
    pub const DISCUSS_LINK: &str = "__DISCUSS_LINK__";
    pub const COMMENT_PREFIX: &str = "__COMMENT_PREFIX___";
    pub const COMMENT_LEADING: &str = "__COMMENT_LEADING__";
    pub const COMMENT_SUFFIX: &str = "__COMMENT_SUFFIX___";
    pub const CODE_START_MARKER: &str = "__CODE_START_MARKER__";
    pub const CODE_END_MARKER: &str = "__CODE_END_MARKER__";

    pub const CODE_START_TEMPLATE: &str = "__COMMENT_PREFIX___
__COMMENT_LEADING__ __CODE_START_MARKER__
__COMMENT_SUFFIX___";
    pub const CODE_END_TEMPLATE: &str = "__COMMENT_PREFIX___
__COMMENT_LEADING__ __CODE_END_MARKER__
__COMMENT_SUFFIX___";
}

fn default_pick() -> String {
    "${fid}.${slug}".into()
}

fn default_submission() -> String {
    "${fid}.${slug}.${sid}.${ac}".into()
}

/// Code config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Code {
    #[serde(default)]
    pub editor: String,
    #[serde(rename(serialize = "editor-args"), alias = "editor-args", default)]
    pub editor_args: Option<Vec<String>>,
    #[serde(rename(serialize = "editor-envs"), alias = "editor-envs", default)]
    pub editor_envs: Option<Vec<String>>,
    #[serde(default)]
    pub start_marker: String,
    #[serde(default)]
    pub end_marker: String,
    #[serde(default)]
    pub comment_prefix: String,
    #[serde(default)]
    pub comment_leading: String,
    #[serde(default)]
    pub comment_suffix: String,
    #[serde(default)]
    pub test: bool,
    pub lang: String,
    #[serde(default = "default_pick", skip_serializing)]
    pub pick: String,
    #[serde(default = "default_submission", skip_serializing)]
    pub submission: String,
}

impl Default for Code {
    fn default() -> Self {
        Self {
            editor: "vim".into(),
            editor_args: None,
            editor_envs: None,
            start_marker: "code start here".into(),
            end_marker: "code end here".into(),
            comment_prefix: "/**".into(),
            comment_leading: " * ".into(),
            comment_suffix: " */".into(),
            test: true,
            lang: "rust".into(),
            pick: "${fid}.${slug}".into(),
            submission: "${fid}.${slug}.${sid}.${ac}".into(),
        }
    }
}
