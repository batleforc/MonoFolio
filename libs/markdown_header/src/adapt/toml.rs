use crate::{Format, Markdown};

use super::Adapter;

/// TomlAdapter
///
/// transform markdown frontMatter into toml
#[derive(Default, Debug)]
pub struct TomlAdapter();

impl Adapter for TomlAdapter {
    fn adapt<T>(&self, mut md: Markdown) -> crate::MarkdownResult
    where
        T: serde::ser::Serialize + serde::de::DeserializeOwned,
    {
        match md.format() {
            Format::TOML => Ok(md),
            Format::JSON | Format::YAML => {
                let data = super::de_fm::<T>(&md)?;

                md.set_front_matter(self.ser_fm(&data)?);
                md.set_format(Format::TOML);
                Ok(md)
            }
        }
    }

    fn ser_fm<T>(&self, data: &T) -> Result<String, crate::Error>
    where
        T: serde::Serialize,
    {
        let mut front_matter = toml::to_string(&data)?;

        if front_matter.ends_with("\n") {
            front_matter.pop();
        }

        Ok(front_matter)
    }
}
