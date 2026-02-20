use crate::error::ReaderError;
use crate::gml_reader::SubtreeReader;

/// Trait for types that can be deserialized from GML XML.
pub trait FromGml: Sized {
    fn from_gml(reader: &mut SubtreeReader<'_>) -> Result<Self, ReaderError>;
}
