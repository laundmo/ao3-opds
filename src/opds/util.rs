use quick_xml::events::BytesText;

use crate::{XmlResult, XmlWriter};
pub trait OpdsEnumStr {
    fn as_str(&self) -> &'static str;
}

pub fn write_elem(name: &str, value: &str, writer: &mut XmlWriter) -> XmlResult {
    writer
        .create_element(name)
        .write_text_content(BytesText::new(value))?;
    Ok(())
}
