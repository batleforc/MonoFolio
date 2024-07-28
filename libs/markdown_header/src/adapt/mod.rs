mod json;
mod toml;
mod yaml;

use crate::{error::Error, Format, Markdown};

pub use self::json::JsonAdapter;
pub use self::toml::TomlAdapter;
pub use self::yaml::YamlAdapter;

/// MDAdapter trait
/// can transform the format of a Markdown into the given kind
pub trait Adapter {
    fn adapt<T>(&self, md: Markdown) -> crate::MarkdownResult
    where
        T: serde::ser::Serialize + serde::de::DeserializeOwned;

    fn ser_fm<T>(&self, data: &T) -> Result<String, Error>
    where
        T: serde::ser::Serialize;
}

/// parse front matter into any struct
///
/// just deserialize it
pub fn de_fm<T>(md: &Markdown) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    let fm = md.front_matter();

    match md.format() {
        Format::YAML => {
            let data = serde_yaml::from_str::<T>(&fm)?;

            return Ok(data);
        }
        Format::TOML => {
            let data = ::toml::from_str::<T>(&fm)?;

            return Ok(data);
        }
        Format::JSON => {
            let data = serde_json::from_str::<T>(&fm)?;

            return Ok(data);
        }
    }
}

/// the all optional format of front matters
///
/// even if unexpected data is given
///
/// it will safely return an none struct
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct BasicObject {
    date: Option<String>,
    /// required field
    title: Option<String>,
    /// optional field
    tags: Option<Vec<String>>,
    /// optional field
    categories: Option<Vec<String>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
/// empty object
///
/// using it for test reason
pub struct EmptyObject {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adapt_mod_basic_object() {
        let basic_object = BasicObject {
            date: Some("2020-01-01".to_string()),
            title: Some("Hello, World!".to_string()),
            tags: Some(vec!["hello".to_string(), "world".to_string()]),
            categories: Some(vec!["hello".to_string(), "world".to_string()]),
        };

        let basic_object_str = serde_json::to_string(&basic_object).unwrap();
        let basic_object_deser: BasicObject = serde_json::from_str(&basic_object_str).unwrap();

        assert_eq!(basic_object, basic_object_deser);
    }

    #[test]
    fn test_adapt_mod_empty_object() {
        let empty_object = EmptyObject {};

        let empty_object_str = serde_json::to_string(&empty_object).unwrap();
        let empty_object_deser: EmptyObject = serde_json::from_str(&empty_object_str).unwrap();

        assert_eq!(empty_object, empty_object_deser);
    }
}
