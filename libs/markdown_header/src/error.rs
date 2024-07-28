use std::borrow::Cow;

use quick_error::quick_error;

use crate::Format;

#[derive(Debug)]
pub enum ParseError {
    BadFormat(Format),
    MissingAllFormat,
}

quick_error! {
    #[derive(Debug)]
    pub enum SerdeError {
        JsonError(err: serde_json::Error) {
            from()
        }

        YamlError(err: serde_yaml::Error) {
            from()
        }

        TomlSerError(err: toml::ser::Error) {
            from()
        }

        TomlDeError(err: toml::de::Error) {
            from()
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error  {

        IO(err: std::io::Error) {
            from()
        }

        Regex(err: regex::Error) {
            from()
        }

        Parse(err: ParseError) {
            from()
        }

        Serde(err: SerdeError) {
            from()
            from(err: toml::de::Error) -> (SerdeError::from(err))
            from(err: toml::ser::Error) -> (SerdeError::from(err))
            from(err: serde_json::Error) -> (SerdeError::from(err))
            from(err: serde_yaml::Error) -> (SerdeError::from(err))
        }

        Unexpected(err: Cow<'static, str>) {
            from()
        }
    }
}
