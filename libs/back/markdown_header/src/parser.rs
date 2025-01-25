use enum_iterator::{all, Sequence};
use getset::{Getters, Setters};

use crate::{error::Error, Adapter};

pub type MarkdownResult = Result<Markdown, Error>;

/// Markdown
/// it is a struct refer a true md file
///
/// including `format`,`content` and `front_matter`
#[derive(Debug, Getters, Setters, Clone)]
pub struct Markdown {
    #[getset(get = "pub", set = "pub")]
    content: String,
    #[getset(get = "pub", set = "pub")]
    front_matter: String,
    #[getset(get = "pub", set = "pub")]
    format: Format,
}

impl Markdown {
    #[inline]
    pub fn new(content: String, front_matter: String, format: Format) -> Self {
        Markdown {
            format,
            content,
            front_matter,
        }
    }

    #[inline]
    /// write data into `Vec<u8>`
    pub fn bytes(&self) -> Vec<u8> {
        let mut v = vec![];
        let sp = self.format().separator();
        v.extend_from_slice(sp.as_bytes());
        v.extend_from_slice(self.front_matter().as_bytes());
        v.extend_from_slice(b"\n");
        v.extend_from_slice(sp.as_bytes());
        v.extend_from_slice(b"\n");
        v.extend_from_slice(self.content().as_bytes());

        v
    }

    #[inline]
    /// display all data as md format into a String
    pub fn display(&self) -> String {
        unsafe { String::from_utf8_unchecked(self.bytes()) }
    }

    /// transform a Markdown struct into another format
    ///
    /// require two types: Data Object and Adapter
    /// # Examples
    ///```
    /// use markdown_header::*;
    /// use markdown_header::adapt::{JsonAdapter};
    /// use std::fs;
    /// #[derive(serde::Serialize, serde::Deserialize, Debug)]
    /// struct SafeFM {
    ///   title: String,
    /// }
    /// let md = Markdown::new(
    ///     "content".to_string(),
    ///     "{title &ésd hello".to_string(),
    ///     Format::JSON,
    /// );
    /// let md = md.adapt::<JsonAdapter, SafeFM>();
    ///```
    ///
    pub fn adapt<A, T>(self) -> MarkdownResult
    where
        A: Adapter + Default,
        T: serde::ser::Serialize + serde::de::DeserializeOwned,
    {
        A::default().adapt::<T>(self)
    }
}

/// format of format matters
///
/// - json
/// - yaml
/// - toml
#[derive(Debug, Clone, PartialEq, Sequence)]
pub enum Format {
    JSON,
    YAML,
    TOML,
}

impl Format {
    #[inline]
    /// internal format separator
    fn separator(&self) -> &str {
        match self {
            Format::YAML => "---\n",
            Format::TOML => "+++\n",
            _ => "",
        }
    }

    #[inline]
    /// internal format regex patten
    fn regex_patten(&self) -> &str {
        match self {
            Format::YAML => r"^[[:space:]]*\-\-\-\r?\n((?s).*?(?-s))\-\-\-\r?\n((?s).*(?-s))$",
            Format::TOML => r"^[[:space:]]*\+\+\+\r?\n((?s).*?(?-s))\+\+\+\r?\n((?s).*(?-s))$",
            Format::JSON => r"^[[:space:]]*\{\r?\n((?s).*?(?-s))\}\r?\n((?s).*(?-s))$",
        }
    }
}

/// parse data and guess the `Format`
pub fn parse(input: &str) -> MarkdownResult {
    for format in all::<Format>() {
        let md = parse_format(input, format);
        if md.is_ok() {
            return md;
        }
    }

    Err(crate::ParseError::MissingAllFormat.into())
}

/// parse data to the given `Format`
pub fn parse_format(input: &str, format: Format) -> MarkdownResult {
    let cap = regex::Regex::new(format.regex_patten())?
        .captures(input)
        .ok_or(crate::ParseError::BadFormat(format.clone()))?;

    // json should have `{` and `}`
    let front_matter = if Format::JSON.eq(&format) {
        format!("{{\n{0}\n}}", cap[1].trim())
    } else {
        cap[1].trim().to_string()
    };

    Ok(Markdown {
        format,
        front_matter,
        content: cap[2].trim().to_string(),
    })
}

#[cfg(test)]
mod tests {
    use crate::{JsonAdapter, TomlAdapter, YamlAdapter};

    use super::*;

    #[test]
    fn test_parser_parse_format_json() {
        let input = r#"
        {
        "title": "hello"
        }
        # Hello
        "#;
        let md = parse_format(input, Format::JSON).unwrap();
        assert_eq!(md.front_matter(), "{\n\"title\": \"hello\"\n}");
        assert_eq!(md.content(), "# Hello");
    }

    #[test]
    fn test_parser_parse_format_yaml() {
        let input = r#"
        ---
        title: hello
        ---
        # Hello
        "#;
        let md = parse_format(input, Format::YAML).unwrap();
        assert_eq!(md.front_matter(), "title: hello");
        assert_eq!(md.content(), "# Hello");
    }

    #[test]
    fn test_parser_parse_format_toml() {
        let input = r#"
        +++
        title = "hello"
        +++
        # Hello
        "#;
        let md = parse_format(input, Format::TOML).unwrap();
        assert_eq!(md.front_matter(), "title = \"hello\"");
        assert_eq!(md.content(), "# Hello");
    }

    #[test]
    fn test_parser_parse_yaml() {
        let input = r#"
        ---
        title: hello
        ---
        # Hello
        "#;
        let md = parse(input).unwrap();
        assert_eq!(md.front_matter(), "title: hello");
        assert_eq!(md.content(), "# Hello");
    }

    #[test]
    fn test_parser_parse_error() {
        let input = r#"
        title: hello
        ---
        # Hello
        "#;
        let md = parse(input);
        assert!(md.is_err());
    }

    #[test]
    fn test_parser_markdown_new() {
        let mut md = Markdown::new(
            "content".to_string(),
            "front_matter".to_string(),
            Format::JSON,
        );
        assert_eq!(md.content(), "content");
        assert_eq!(md.front_matter(), "front_matter");
        assert_eq!(md.format(), &Format::JSON);

        md.set_content("new_content".to_string());
        assert_eq!(md.content(), "new_content");
        md.set_format(Format::YAML);
        assert_eq!(md.format(), &Format::YAML);
        md.set_front_matter("new_front_matter".to_string());
        assert_eq!(md.front_matter(), "new_front_matter");
    }

    #[test]
    fn test_parser_markdown_bytes() {
        let md = Markdown::new(
            "content".to_string(),
            "front_matter".to_string(),
            Format::JSON,
        );
        let bytes = md.bytes();
        assert_eq!(bytes, b"front_matter\n\ncontent");
    }

    #[test]
    fn test_parser_markdown_display() {
        let md = Markdown::new(
            "content".to_string(),
            "front_matter".to_string(),
            Format::JSON,
        );
        let display = md.display();
        assert_eq!(display, "front_matter\n\ncontent");
    }

    #[test]
    fn test_parser_format_separator() {
        assert_eq!(Format::YAML.separator(), "---\n");
        assert_eq!(Format::TOML.separator(), "+++\n");
        assert_eq!(Format::JSON.separator(), "");
    }

    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    struct SafeFM {
        title: String,
    }
    #[test]
    fn test_parser_markdown_adapt_yaml_to_json() {
        let md = Markdown::new(
            "content".to_string(),
            "title: hello".to_string(),
            Format::YAML,
        );
        let md = md.adapt::<JsonAdapter, SafeFM>().unwrap();
        assert_eq!(md.front_matter(), "{\n  \"title\": \"hello\"\n}");
        assert_eq!(md.format(), &Format::JSON);
    }

    #[test]
    fn test_parser_markdown_adapt_json_to_yaml() {
        let md = Markdown::new(
            "content".to_string(),
            "{\n  \"title\": \"hello\"\n}".to_string(),
            Format::JSON,
        );
        let md = md.adapt::<YamlAdapter, SafeFM>().unwrap();
        assert_eq!(md.front_matter(), "title: hello");
        assert_eq!(md.format(), &Format::YAML);
    }

    #[test]
    fn test_parser_markdown_adapt_json_to_json() {
        let md = Markdown::new(
            "content".to_string(),
            "{\n  \"title\": \"hello\"\n}".to_string(),
            Format::JSON,
        );
        let md = md.adapt::<JsonAdapter, SafeFM>().unwrap();
        assert_eq!(md.front_matter(), "{\n  \"title\": \"hello\"\n}");
        assert_eq!(md.format(), &Format::JSON);
    }

    #[test]
    fn test_parser_makdown_adapt_toml_to_toml() {
        let md = Markdown::new(
            "content".to_string(),
            "title = \"hello\"".to_string(),
            Format::TOML,
        );
        let md = md.adapt::<TomlAdapter, SafeFM>().unwrap();
        assert_eq!(md.front_matter(), "title = \"hello\"");
        assert_eq!(md.format(), &Format::TOML);
    }

    #[test]
    fn test_parser_markdown_adapt_yaml_to_yaml() {
        let md = Markdown::new(
            "content".to_string(),
            "title: hello".to_string(),
            Format::YAML,
        );
        let md = md.adapt::<YamlAdapter, SafeFM>().unwrap();
        assert_eq!(md.front_matter(), "title: hello");
        assert_eq!(md.format(), &Format::YAML);
    }

    #[test]
    fn test_parser_markdown_adapt_json_to_toml() {
        let md = Markdown::new(
            "content".to_string(),
            "{\n  \"title\": \"hello\"\n}".to_string(),
            Format::JSON,
        );
        let md = md.adapt::<TomlAdapter, SafeFM>().unwrap();
        assert_eq!(md.front_matter(), "title = \"hello\"");
        assert_eq!(md.format(), &Format::TOML);
    }

    #[test]
    fn test_parser_makdown_adapt_toml_to_json() {
        let md = Markdown::new(
            "content".to_string(),
            "title = \"hello\"".to_string(),
            Format::TOML,
        );
        let md = md.adapt::<JsonAdapter, SafeFM>().unwrap();
        assert_eq!(md.front_matter(), "{\n  \"title\": \"hello\"\n}");
        assert_eq!(md.format(), &Format::JSON);
    }

    #[test]
    fn test_parser_makdown_adapt_yaml_to_json_invalid() {
        let md = Markdown::new(
            "content".to_string(),
            "title= hello".to_string(),
            Format::YAML,
        );
        let md = md.adapt::<JsonAdapter, SafeFM>();
        assert!(md.is_err());
    }

    #[test]
    fn test_parser_makdown_adapt_json_to_yaml_invalid() {
        let md = Markdown::new(
            "content".to_string(),
            "{title= hello".to_string(),
            Format::JSON,
        );
        let md = md.adapt::<YamlAdapter, SafeFM>();
        assert!(md.is_err());
    }

    #[test]
    fn test_parser_makdown_adapt_toml_to_json_invalid() {
        let md = Markdown::new(
            "content".to_string(),
            "title &ésd hello".to_string(),
            Format::TOML,
        );
        let md = md.adapt::<JsonAdapter, SafeFM>();
        assert!(md.is_err());
    }

    #[test]
    fn test_parser_makdown_adapt_json_to_toml_invalid() {
        let md = Markdown::new(
            "content".to_string(),
            "{title &ésd hello".to_string(),
            Format::JSON,
        );
        let md = md.adapt::<TomlAdapter, SafeFM>();
        assert!(md.is_err());
    }
}
