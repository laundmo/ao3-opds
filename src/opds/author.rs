use crate::XmlWriter;

use color_eyre::Result;
use quick_xml::events::BytesText;

/// Represents an author in an OPDS feed.
pub struct StumpAuthor {
    pub name: String,
    pub uri: Option<String>,
}

impl StumpAuthor {
    pub fn new(name: String, uri: Option<String>) -> StumpAuthor {
        StumpAuthor { name, uri }
    }

    pub fn write(&self, writer: &mut XmlWriter) -> Result<()> {
        writer
            .create_element("author")
            .write_inner_content(|writer| {
                writer
                    .create_element("name")
                    .write_text_content(BytesText::new(&self.name))?;
                if let Some(uri) = &self.uri {
                    writer
                        .create_element("uri")
                        .write_text_content(BytesText::new(uri))?;
                }
                Ok(())
            })
            .expect("failure");
        Ok(())
    }
}
