use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReaderError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("XML parse error: {0}")]
    Xml(#[from] quick_xml::Error),

    #[error("XML attribute error: {0}")]
    XmlAttr(#[from] quick_xml::events::attributes::AttrError),

    #[error("Unsupported feature type: {namespace}:{local_name}")]
    UnsupportedFeature {
        namespace: String,
        local_name: String,
    },

    #[error("Missing required element: {element} in {parent}")]
    MissingElement {
        element: String,
        parent: String,
    },

    #[error("Parse error: {message}")]
    Parse { message: String },

    #[error("Unexpected XML structure: {0}")]
    UnexpectedStructure(String),
}
