use std::collections::HashMap;
use std::io::BufRead;

use quick_xml::events::Event;
use quick_xml::Reader;

use crate::error::ReaderError;

/// Information about an XML element after namespace resolution.
#[derive(Debug, Clone)]
pub struct ElementInfo {
    pub namespace: String,
    pub local_name: String,
    pub attributes: Vec<(String, String)>,
}

/// Extract namespace declarations and regular attributes from a BytesStart event.
fn extract_attrs_and_ns(
    event: &quick_xml::events::BytesStart<'_>,
) -> Result<(HashMap<String, String>, Vec<(String, String)>), ReaderError> {
    let mut ns_decls = HashMap::new();
    let mut attrs = Vec::new();

    for attr_result in event.attributes() {
        let attr = attr_result?;
        let key = std::str::from_utf8(attr.key.as_ref())
            .map_err(|e| ReaderError::Parse {
                message: e.to_string(),
            })?
            .to_string();
        let value = String::from_utf8(attr.value.to_vec()).map_err(|e| ReaderError::Parse {
            message: e.to_string(),
        })?;

        if key == "xmlns" {
            ns_decls.insert(String::new(), value);
        } else if let Some(prefix) = key.strip_prefix("xmlns:") {
            ns_decls.insert(prefix.to_string(), value);
        } else {
            attrs.push((key, value));
        }
    }

    Ok((ns_decls, attrs))
}

/// Resolve a qualified name against the namespace stack.
fn resolve_name(ns_stack: &[HashMap<String, String>], qname: &str) -> (String, String) {
    if let Some((prefix, local)) = qname.split_once(':') {
        let uri = lookup_ns(ns_stack, prefix).unwrap_or_default();
        (uri, local.to_string())
    } else {
        let uri = lookup_ns(ns_stack, "").unwrap_or_default();
        (uri, qname.to_string())
    }
}

/// Look up a namespace prefix in the stack (most recent first).
fn lookup_ns(ns_stack: &[HashMap<String, String>], prefix: &str) -> Option<String> {
    for frame in ns_stack.iter().rev() {
        if let Some(uri) = frame.get(prefix) {
            return Some(uri.clone());
        }
    }
    None
}

/// Build an ElementInfo from a BytesStart event and pre-extracted attributes.
fn build_element_info(
    ns_stack: &[HashMap<String, String>],
    event: &quick_xml::events::BytesStart<'_>,
    extra_attrs: Vec<(String, String)>,
) -> Result<ElementInfo, ReaderError> {
    let qname = std::str::from_utf8(event.name().as_ref())
        .map_err(|e| ReaderError::Parse {
            message: e.to_string(),
        })?
        .to_string();
    let (namespace, local_name) = resolve_name(ns_stack, &qname);
    Ok(ElementInfo {
        namespace,
        local_name,
        attributes: extra_attrs,
    })
}

/// Internal state shared between GmlReader and SubtreeReader.
struct GmlReaderInner {
    reader: Reader<Box<dyn BufRead>>,
    buf: Vec<u8>,
    ns_stack: Vec<HashMap<String, String>>,
    depth: u32,
    last_was_empty: bool,
}

/// A namespace-aware XML reader wrapping quick_xml.
pub struct GmlReader {
    inner: GmlReaderInner,
}

impl GmlReader {
    pub fn new(reader: Box<dyn BufRead>) -> Self {
        let mut xml = Reader::from_reader(reader);
        xml.config_mut().trim_text(true);
        GmlReader {
            inner: GmlReaderInner {
                reader: xml,
                buf: Vec::with_capacity(8192),
                ns_stack: vec![HashMap::new()],
                depth: 0,
                last_was_empty: false,
            },
        }
    }

    /// Advance to the next Start or Empty element, returning its info.
    pub fn next_start_element(&mut self) -> Result<Option<ElementInfo>, ReaderError> {
        self.inner.next_start_element()
    }

    /// Create a subtree reader scoped to the current element's children.
    pub fn subtree(&mut self) -> SubtreeReader<'_> {
        let target_depth = self.inner.depth;
        let is_empty = self.inner.last_was_empty;
        SubtreeReader {
            inner: &mut self.inner,
            target_depth,
            finished: is_empty,
        }
    }
}

impl GmlReaderInner {
    fn pop_empty_if_needed(&mut self) {
        if self.last_was_empty {
            if self.depth > 0 {
                self.depth -= 1;
                if self.ns_stack.len() > 1 {
                    self.ns_stack.pop();
                }
            }
            self.last_was_empty = false;
        }
    }

    fn next_start_element(&mut self) -> Result<Option<ElementInfo>, ReaderError> {
        loop {
            self.buf.clear();
            let event = self.reader.read_event_into(&mut self.buf)?;
            match event {
                Event::Start(ref e) => {
                    let (ns_decls, attrs) = extract_attrs_and_ns(e)?;
                    self.ns_stack.push(ns_decls);
                    self.depth += 1;
                    self.last_was_empty = false;
                    let info = build_element_info(&self.ns_stack, e, attrs)?;
                    return Ok(Some(info));
                }
                Event::Empty(ref e) => {
                    let (ns_decls, attrs) = extract_attrs_and_ns(e)?;
                    self.ns_stack.push(ns_decls);
                    self.depth += 1;
                    self.last_was_empty = true;
                    let info = build_element_info(&self.ns_stack, e, attrs)?;
                    return Ok(Some(info));
                }
                Event::End(_) => {
                    if self.depth > 0 {
                        self.depth -= 1;
                        if self.ns_stack.len() > 1 {
                            self.ns_stack.pop();
                        }
                    }
                    self.last_was_empty = false;
                }
                Event::Text(_) | Event::CData(_) | Event::Comment(_) | Event::PI(_)
                | Event::Decl(_) | Event::DocType(_) => continue,
                Event::Eof => return Ok(None),
            }
        }
    }
}

/// A scoped reader for iterating over the children of a single element.
pub struct SubtreeReader<'a> {
    inner: &'a mut GmlReaderInner,
    target_depth: u32,
    finished: bool,
}

impl<'a> SubtreeReader<'a> {
    /// Advance to the next child Start/Empty element.
    pub fn next_element(&mut self) -> Result<Option<ElementInfo>, ReaderError> {
        if self.finished {
            return Ok(None);
        }
        // Clean up depth from any previous Empty element that wasn't
        // consumed by skip_element/read_text.  This happens when the caller
        // creates a subtree() from an Empty element (immediately finished)
        // and then comes back to call next_element() again.
        self.inner.pop_empty_if_needed();
        loop {
            self.inner.buf.clear();
            let event = self.inner.reader.read_event_into(&mut self.inner.buf)?;
            match event {
                Event::Start(ref e) => {
                    let (ns_decls, attrs) = extract_attrs_and_ns(e)?;
                    self.inner.ns_stack.push(ns_decls);
                    self.inner.depth += 1;
                    self.inner.last_was_empty = false;
                    let info = build_element_info(&self.inner.ns_stack, e, attrs)?;
                    return Ok(Some(info));
                }
                Event::Empty(ref e) => {
                    let (ns_decls, attrs) = extract_attrs_and_ns(e)?;
                    self.inner.ns_stack.push(ns_decls);
                    self.inner.depth += 1;
                    self.inner.last_was_empty = true;
                    let info = build_element_info(&self.inner.ns_stack, e, attrs)?;
                    return Ok(Some(info));
                }
                Event::End(_) => {
                    if self.inner.depth > 0 {
                        self.inner.depth -= 1;
                        if self.inner.ns_stack.len() > 1 {
                            self.inner.ns_stack.pop();
                        }
                    }
                    self.inner.last_was_empty = false;
                    if self.inner.depth < self.target_depth {
                        self.finished = true;
                        return Ok(None);
                    }
                }
                Event::Text(_) | Event::CData(_) | Event::Comment(_) | Event::PI(_)
                | Event::Decl(_) | Event::DocType(_) => continue,
                Event::Eof => {
                    self.finished = true;
                    return Ok(None);
                }
            }
        }
    }

    /// Read the text content of the current element.
    pub fn read_text(&mut self) -> Result<String, ReaderError> {
        if self.inner.last_was_empty {
            self.inner.pop_empty_if_needed();
            return Ok(String::new());
        }
        let mut text = String::new();
        let target = self.inner.depth;
        loop {
            self.inner.buf.clear();
            let event = self.inner.reader.read_event_into(&mut self.inner.buf)?;
            match event {
                Event::Text(ref e) => {
                    text.push_str(
                        &e.unescape()
                            .map_err(|e| ReaderError::Parse {
                                message: e.to_string(),
                            })?,
                    );
                }
                Event::CData(ref e) => {
                    text.push_str(
                        std::str::from_utf8(e.as_ref())
                            .map_err(|e| ReaderError::Parse {
                                message: e.to_string(),
                            })?,
                    );
                }
                Event::End(_) => {
                    if self.inner.depth > 0 {
                        self.inner.depth -= 1;
                        if self.inner.ns_stack.len() > 1 {
                            self.inner.ns_stack.pop();
                        }
                    }
                    self.inner.last_was_empty = false;
                    if self.inner.depth < target {
                        return Ok(text);
                    }
                }
                Event::Start(ref e) => {
                    let (ns_decls, _) = extract_attrs_and_ns(e)?;
                    self.inner.ns_stack.push(ns_decls);
                    self.inner.depth += 1;
                    self.inner.last_was_empty = false;
                }
                Event::Empty(_) => {}
                Event::Eof => return Ok(text),
                _ => {}
            }
        }
    }

    /// Skip the current element and all its children.
    pub fn skip_element(&mut self) -> Result<(), ReaderError> {
        if self.inner.last_was_empty {
            self.inner.pop_empty_if_needed();
            return Ok(());
        }
        let target = self.inner.depth;
        loop {
            self.inner.buf.clear();
            let event = self.inner.reader.read_event_into(&mut self.inner.buf)?;
            match event {
                Event::Start(ref e) => {
                    let (ns_decls, _) = extract_attrs_and_ns(e)?;
                    self.inner.ns_stack.push(ns_decls);
                    self.inner.depth += 1;
                    self.inner.last_was_empty = false;
                }
                Event::Empty(_) => {}
                Event::End(_) => {
                    if self.inner.depth > 0 {
                        self.inner.depth -= 1;
                        if self.inner.ns_stack.len() > 1 {
                            self.inner.ns_stack.pop();
                        }
                    }
                    self.inner.last_was_empty = false;
                    if self.inner.depth < target {
                        return Ok(());
                    }
                }
                Event::Eof => return Ok(()),
                _ => {}
            }
        }
    }

    /// Create a nested subtree reader for the element that was just entered.
    pub fn subtree(&mut self) -> SubtreeReader<'_> {
        let target_depth = self.inner.depth;
        let is_empty = self.inner.last_was_empty;
        SubtreeReader {
            inner: self.inner,
            target_depth,
            finished: is_empty,
        }
    }

    /// Get the gml:id attribute from the element.
    pub fn gml_id(info: &ElementInfo) -> Option<String> {
        info.attributes
            .iter()
            .find(|(k, _)| k == "gml:id" || k == "id")
            .map(|(_, v)| v.clone())
    }
}

/// Create a GmlReader from a byte slice.
pub fn reader_from_bytes(data: &[u8]) -> GmlReader {
    let owned: Vec<u8> = data.to_vec();
    let cursor = std::io::Cursor::new(owned);
    GmlReader::new(Box::new(std::io::BufReader::new(cursor)))
}

/// Create a GmlReader from a file path.
pub fn reader_from_path(path: &std::path::Path) -> Result<GmlReader, ReaderError> {
    let file = std::fs::File::open(path)?;
    let buf = std::io::BufReader::new(file);
    Ok(GmlReader::new(Box::new(buf)))
}
