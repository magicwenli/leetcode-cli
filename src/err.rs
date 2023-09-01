//! Errors in leetcode-cli
use crate::cmds::{Command, DataCommand};
use colored::Colorize;
use std::{fmt, string::FromUtf8Error};

// fixme: use this_error
/// Error enum
pub enum Error {
    MatchError,
    DownloadError(String),
    NetworkError(String),
    ParseError(String),
    CacheError(String),
    FeatureError(String),
    ScriptError(String),
    CookieError,
    PremiumError,
    DecryptError,
    SilentError,
    Utf8ParseError,
    NoneError,
    ChromeNotLogin,
    Anyhow(anyhow::Error),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let e = "error:".bold().red();
        match self {
            Error::CacheError(s) => write!(f, "{} {}, please try again", e, s),
            Error::CookieError => write!(
                f,
                "{} \
                 Your leetcode cookies seems expired, \
                 {} \
                 Either you can handwrite your `LEETCODE_SESSION` and `csrf` into `leetcode.toml`, \
                 more info please checkout this: \
                 https://github.com/clearloop/leetcode-cli/blob/master/README.md#cookies",
                e,
                "please make sure you have logined in leetcode.cn with chrome. "
                    .yellow()
                    .bold(),
            ),
            Error::PremiumError => write!(
                f,
                "{} \
                Your leetcode account lacks a premium subscription, which the given problem requires.\n \
                If this looks like a mistake, please open a new issue at: {}",
                e,
                "https://github.com/clearloop/leetcode-cli/".underline()),
            Error::DownloadError(s) => write!(f, "{} Download {} failed, please try again", e, s),
            Error::NetworkError(s) => write!(f, "{} {}, please try again", e, s),
            Error::ParseError(s) => write!(f, "{} {}", e, s),
            Error::FeatureError(s) => write!(f, "{} {}", e, s),
            Error::MatchError => write!(f, "{} Nothing matches", e),
            Error::DecryptError => write!(f, "{} openssl decrypt failed", e),
            Error::ScriptError(s) => write!(f, "{} {}", e, s),
            Error::SilentError => write!(f, ""),
            Error::NoneError => panic!(
                "{} {}",
                e,
                "this is a bug, please report it to"
            ),
            Error::ChromeNotLogin => write!(f, "maybe you not login on the Chrome, you can login and retry."),
            Error::Anyhow(e) => write!(f, "{} {}", e, e),
            Error::Utf8ParseError => write!(f, "cannot parse utf8 from buff {}", e),
        }
    }
}

// network
impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::NetworkError(err.to_string())
    }
}

// utf8 parse
impl std::convert::From<FromUtf8Error> for Error {
    fn from(_err: FromUtf8Error) -> Self {
        Error::Utf8ParseError
    }
}

// nums
impl std::convert::From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::ParseError(err.to_string())
    }
}

// sql
impl std::convert::From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => {
                println!("NotFound, you may update cache, and try it again\r\n");
                DataCommand::usage().print_help().unwrap_or(());
                Error::SilentError
            }
            _ => Error::CacheError(err.to_string()),
        }
    }
}

// serde
impl std::convert::From<serde_json::error::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::ParseError(err.to_string())
    }
}

// toml
impl std::convert::From<toml::de::Error> for Error {
    fn from(_err: toml::de::Error) -> Self {
        #[cfg(debug_assertions)]
        let err_msg = format!(
            "{}, {}{}{}{}{}{}",
            _err,
            "Parse config file failed, ",
            "leetcode-cli has just generated a new leetcode.toml at ",
            "~/.leetcode/leetcode.tmp.toml,".green().bold().underline(),
            " the current one at ",
            "~/.leetcode/leetcode.toml".yellow().bold().underline(),
            " seems missing some keys, Please compare the new file and add the missing keys.\n",
        );
        #[cfg(not(debug_assertions))]
        let err_msg = format!(
            "{}{}{}{}{}{}",
            "Parse config file failed, ",
            "leetcode-cli has just generated a new leetcode.toml at ",
            "~/.leetcode/leetcode_tmp.toml,".green().bold().underline(),
            " the current one at ",
            "~/.leetcode/leetcode.toml".yellow().bold().underline(),
            " seems missing some keys, Please compare the new file and add the missing keys.\n",
        );
        Error::ParseError(err_msg.trim_start().into())
    }
}

impl std::convert::From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Error::ParseError(err.to_string())
    }
}

// io
impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::CacheError(err.to_string())
    }
}

// openssl
impl std::convert::From<openssl::error::ErrorStack> for Error {
    fn from(_: openssl::error::ErrorStack) -> Self {
        Error::DecryptError
    }
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Anyhow(err)
    }
}

// pyo3
#[cfg(feature = "pym")]
impl std::convert::From<pyo3::PyErr> for Error {
    fn from(_: pyo3::PyErr) -> Self {
        Error::ScriptError("Python script went Error".to_string())
    }
}
