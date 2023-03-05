use serde::{self, Serialize};

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

impl ToString for OpdsLinkType {
    fn to_string(&self) -> String {
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
        .to_string()
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

impl ToString for OpdsLinkRel {
    fn to_string(&self) -> String {
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
        .to_string()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OpdsLink {
    #[serde(rename = "@link")]
    link_type: String,
    #[serde(rename = "@rel")]
    rel: String,
    #[serde(rename = "@href")]
    pub href: String,
}

impl OpdsLink {
    pub fn new(link_type: OpdsLinkType, rel: OpdsLinkRel, href: String) -> Self {
        Self {
            link_type: link_type.to_string(),
            rel: rel.to_string(),
            href,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::opds::{OpdsLink, OpdsLinkRel, OpdsLinkType};

    #[test]
    fn it_works() {
        let v = OpdsLink::new(
            OpdsLinkType::Acquisition,
            OpdsLinkRel::ItSelf,
            "test".to_string(),
        );
        dbg!(quick_xml::se::to_string(&v));
    }
}
