use crate::{XmlResult, XmlWriter};

use super::util::OpdsEnumStr;

#[derive(Debug, Clone, Copy)]
pub enum OpdsLinkType {
    Acquisition, // "application/atom+xml;profile=opds-catalog;kind=acquisition",
    Image,       // "image/jpeg",
    Navigation,  // "application/atom+xml;profile=opds-catalog;kind=navigation",
    OctetStream, // "application/octet-stream",
    Zip,         // "application/zip"
    Epub,        // "application/epub+zip"
    Search,      // "application/opensearchdescription+xml"
}

impl OpdsEnumStr for OpdsLinkType {
    fn as_str(&self) -> &'static str {
        match self {
            OpdsLinkType::Acquisition => {
                "application/atom+xml;profile=opds-catalog;kind=acquisition"
            }
            OpdsLinkType::Image => "image/jpeg",
            OpdsLinkType::Navigation => "application/atom+xml;profile=opds-catalog;kind=navigation",
            OpdsLinkType::OctetStream => "application/octet-stream",
            OpdsLinkType::Zip => "application/zip",
            OpdsLinkType::Epub => "application/epub+zip",
            OpdsLinkType::Search => "application/opensearchdescription+xml",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OpdsLinkRel {
    ItSelf,      // self
    Subsection,  // "subsection",
    Acquisition, // "http://opds-spec.org/acquisition",
    Start,       // start
    Next,        // next
    Previous,    // previous
    Thumbnail,   // "http://opds-spec.org/image/thumbnail"
    Image,       // "http://opds-spec.org/image"
    PageStream,  // "http://vaemendis.net/opds-pse/stream"
    Search,      // "search"
}

impl OpdsEnumStr for OpdsLinkRel {
    fn as_str(&self) -> &'static str {
        match self {
            OpdsLinkRel::ItSelf => "self",
            OpdsLinkRel::Subsection => "subsection",
            OpdsLinkRel::Acquisition => "http://opds-spec.org/acquisition",
            OpdsLinkRel::Start => "start",
            OpdsLinkRel::Next => "next",
            OpdsLinkRel::Previous => "previous",
            OpdsLinkRel::Thumbnail => "http://opds-spec.org/image/thumbnail",
            OpdsLinkRel::Image => "http://opds-spec.org/image",
            OpdsLinkRel::PageStream => "http://vaemendis.net/opds-pse/stream",
            OpdsLinkRel::Search => "search",
        }
    }
}

// TODO: this struct needs to be restructured.
// I need to be able to output the following:
// <link xmlns:wstxns3="http://vaemendis.net/opds-pse/ns" href="/opds/v1.2/books/<id>/pages/{pageNumber}?zero_based=true" wstxns3:count="309" type="image/jpeg" rel="http://vaemendis.net/opds-pse/stream"/>

#[derive(Debug, Clone)]
pub struct OpdsLink {
    pub link_type: OpdsLinkType,
    pub rel: OpdsLinkRel,
    pub href: String,
}

impl OpdsLink {
    pub fn new(link_type: OpdsLinkType, rel: OpdsLinkRel, href: String) -> Self {
        Self {
            link_type,
            rel,
            href,
        }
    }

    pub fn write(&self, writer: &mut XmlWriter) -> XmlResult {
        writer
            .create_element("link")
            .with_attribute(("type", self.link_type.as_str()))
            .with_attribute(("rel", self.rel.as_str()))
            .with_attribute(("href", self.href.as_str()));
        Ok(())
    }
}
