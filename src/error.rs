use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("XML parse error: {0}")]
    Xml(#[from] quick_xml::Error),

    #[error("XML attribute error: {0}")]
    XmlAttr(#[from] quick_xml::events::attributes::AttrError),

    #[error("UTF-8 conversion error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Codegen error: {0}")]
    Codegen(String),

    #[error("Resolution error: {0}")]
    Resolution(String),

    #[error("Parse error at {context}: {message}")]
    Parse { context: String, message: String },
}
