use crate::{Format, Markdown};

use super::Adapter;

/// JsonAdapter
///
/// transform markdown frontMatter into json
#[derive(Default, Debug)]
pub struct JsonAdapter();

impl Adapter for JsonAdapter {
    fn adapt<T>(&self, mut md: Markdown) -> crate::MarkdownResult
    where
        T: serde::ser::Serialize + serde::de::DeserializeOwned,
    {
        match md.format() {
            Format::JSON => Ok(md),
            Format::YAML | Format::TOML => {
                let data = super::de_fm::<T>(&md)?;
                md.set_front_matter(self.ser_fm(&data)?);
                md.set_format(Format::JSON);
                Ok(md)
            }
        }
    }

    fn ser_fm<T>(&self, data: &T) -> Result<String, crate::Error>
    where
        T: serde::Serialize,
    {
        let mut front_matter = serde_json::to_string_pretty(&data)?;

        if front_matter.ends_with('\n') {
            front_matter.pop();
        }

        Ok(front_matter)
    }
}
