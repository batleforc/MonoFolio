pub mod adapt;
mod error;
mod parser;

pub use adapt::{de_fm, Adapter, BasicObject, EmptyObject, JsonAdapter, TomlAdapter, YamlAdapter};
pub use error::{Error, ParseError};
pub use parser::{parse, parse_format, Format, Markdown, MarkdownResult};
