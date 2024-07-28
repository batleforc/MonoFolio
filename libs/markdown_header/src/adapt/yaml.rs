use crate::{Format, Markdown};

use super::Adapter;

/// YamlAdapter
///
/// transform markdown frontMatter into yaml
#[derive(Default, Debug)]
pub struct YamlAdapter();

impl Adapter for YamlAdapter {
    fn adapt<T>(&self, mut md: Markdown) -> crate::MarkdownResult
    where
        T: serde::ser::Serialize + serde::de::DeserializeOwned,
    {
        match md.format() {
            Format::YAML => Ok(md),
            Format::JSON | Format::TOML => {
                let data = super::de_fm::<T>(&md)?;

                md.set_front_matter(self.ser_fm(&data)?);
                md.set_format(Format::YAML);
                Ok(md)
            }
        }
    }

    fn ser_fm<T>(&self, data: &T) -> Result<String, crate::Error>
    where
        T: serde::Serialize,
    {
        let mut front_matter = serde_yaml::to_string(&data)?;

        if front_matter.starts_with("---\n") {
            front_matter = front_matter[4..].to_string();
        }

        if front_matter.ends_with("\n") {
            front_matter.pop();
        }

        Ok(front_matter)
    }
}
