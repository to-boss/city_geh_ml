use std::path::Path;

use quick_xml::Reader;

use crate::error::GenError;

/// Read an XMI file and return a quick-xml Reader.
/// The `encoding` feature on quick-xml handles `encoding="windows-1252"` automatically
/// from the XML declaration.
pub fn open_xmi(path: &Path) -> Result<Reader<std::io::BufReader<std::fs::File>>, GenError> {
    let file = std::fs::File::open(path)?;
    let buf_reader = std::io::BufReader::new(file);
    let mut reader = Reader::from_reader(buf_reader);
    reader.config_mut().trim_text(true);
    Ok(reader)
}
