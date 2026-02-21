use crate::error::ReaderError;
use crate::geometry::*;
use crate::gml_reader::SubtreeReader;
use crate::namespace::NS_GML;

/// Parse a space-separated list of doubles (gml:posList content).
pub fn parse_pos_list(text: &str) -> Vec<f64> {
    text.split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .collect()
}

/// Parse gml:pos content into a DirectPosition.
pub fn parse_pos(text: &str) -> DirectPosition {
    DirectPosition {
        coordinates: parse_pos_list(text),
    }
}

/// Parse a gml:LinearRing element (reads its children: posList or pos elements).
pub fn parse_linear_ring(reader: &mut SubtreeReader<'_>) -> Result<LinearRing, ReaderError> {
    let mut ring = LinearRing::default();
    let mut sub = reader.subtree();
    while let Some(info) = sub.next_element()? {
        if info.namespace == NS_GML {
            match info.local_name.as_str() {
                "posList" => {
                    let text = sub.read_text()?;
                    ring.pos_list = parse_pos_list(&text);
                }
                "pos" => {
                    let text = sub.read_text()?;
                    let coords = parse_pos_list(&text);
                    ring.pos_list.extend(coords);
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        } else {
            sub.skip_element()?;
        }
    }
    Ok(ring)
}

/// Parse a gml:Polygon element.
pub fn parse_polygon(reader: &mut SubtreeReader<'_>) -> Result<Polygon, ReaderError> {
    let mut polygon = Polygon::default();
    let mut sub = reader.subtree();
    while let Some(info) = sub.next_element()? {
        if info.namespace == NS_GML {
            match info.local_name.as_str() {
                "exterior" => {
                    // Expect a gml:LinearRing child
                    let mut ext_sub = sub.subtree();
                    while let Some(ring_info) = ext_sub.next_element()? {
                        if ring_info.namespace == NS_GML && ring_info.local_name == "LinearRing" {
                            polygon.exterior = Some(parse_linear_ring(&mut ext_sub)?);
                        } else {
                            ext_sub.skip_element()?;
                        }
                    }
                }
                "interior" => {
                    let mut int_sub = sub.subtree();
                    while let Some(ring_info) = int_sub.next_element()? {
                        if ring_info.namespace == NS_GML && ring_info.local_name == "LinearRing" {
                            polygon.interior.push(parse_linear_ring(&mut int_sub)?);
                        } else {
                            int_sub.skip_element()?;
                        }
                    }
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        } else {
            sub.skip_element()?;
        }
    }
    Ok(polygon)
}

/// Parse a surface member that could be a Polygon or a CompositeSurface.
fn parse_surface_element(reader: &mut SubtreeReader<'_>, info: &crate::gml_reader::ElementInfo) -> Result<Vec<Polygon>, ReaderError> {
    if info.namespace == NS_GML {
        match info.local_name.as_str() {
            "Polygon" => {
                let mut polygon = parse_polygon(reader)?;
                polygon.gml_id = SubtreeReader::gml_id(info);
                if let Some(id) = polygon.gml_id.as_ref() {
                    reader.register_polygon(id.clone(), polygon.clone());
                }
                Ok(vec![polygon])
            }
            "CompositeSurface" => {
                let mut polygons = Vec::new();
                let mut sub = reader.subtree();
                while let Some(child) = sub.next_element()? {
                    if child.namespace == NS_GML && child.local_name == "surfaceMember" {
                        if let Some(href) = SubtreeReader::xlink_href(&child) {
                            if let Some(p) = sub.resolve_polygon_href(href) {
                                polygons.push(p);
                            }
                            sub.skip_element()?;
                        } else {
                            let mut member_sub = sub.subtree();
                            while let Some(surf) = member_sub.next_element()? {
                                let mut polys = parse_surface_element(&mut member_sub, &surf)?;
                                polygons.append(&mut polys);
                            }
                        }
                    } else {
                        sub.skip_element()?;
                    }
                }
                Ok(polygons)
            }
            "OrientableSurface" => {
                let mut polygons = Vec::new();
                let mut sub = reader.subtree();
                while let Some(child) = sub.next_element()? {
                    if child.namespace == NS_GML && child.local_name == "baseSurface" {
                        let mut base_sub = sub.subtree();
                        while let Some(surf) = base_sub.next_element()? {
                            let mut polys = parse_surface_element(&mut base_sub, &surf)?;
                            polygons.append(&mut polys);
                        }
                    } else {
                        sub.skip_element()?;
                    }
                }
                Ok(polygons)
            }
            _ => {
                reader.skip_element()?;
                Ok(Vec::new())
            }
        }
    } else {
        reader.skip_element()?;
        Ok(Vec::new())
    }
}

/// Parse a gml:MultiSurface element.
pub fn parse_multi_surface(reader: &mut SubtreeReader<'_>) -> Result<MultiSurface, ReaderError> {
    let mut ms = MultiSurface::default();
    let mut sub = reader.subtree();
    while let Some(info) = sub.next_element()? {
        if info.namespace == NS_GML {
            match info.local_name.as_str() {
                "surfaceMember" => {
                    if let Some(href) = SubtreeReader::xlink_href(&info) {
                        if let Some(p) = sub.resolve_polygon_href(href) {
                            ms.surface_members.push(p);
                        }
                        sub.skip_element()?;
                    } else {
                        let mut member_sub = sub.subtree();
                        while let Some(surf_info) = member_sub.next_element()? {
                            let mut polys = parse_surface_element(&mut member_sub, &surf_info)?;
                            ms.surface_members.append(&mut polys);
                        }
                    }
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        } else {
            sub.skip_element()?;
        }
    }
    Ok(ms)
}

/// Parse a gml:Solid element.
pub fn parse_solid(reader: &mut SubtreeReader<'_>) -> Result<Solid, ReaderError> {
    let mut solid = Solid::default();
    let mut sub = reader.subtree();
    while let Some(info) = sub.next_element()? {
        if info.namespace == NS_GML {
            match info.local_name.as_str() {
                "exterior" => {
                    // Contains a gml:Shell or gml:CompositeSurface
                    let mut ext_sub = sub.subtree();
                    while let Some(shell_info) = ext_sub.next_element()? {
                        if shell_info.namespace == NS_GML {
                            match shell_info.local_name.as_str() {
                                "Shell" | "CompositeSurface" => {
                                    let mut shell_sub = ext_sub.subtree();
                                    while let Some(member) = shell_sub.next_element()? {
                                        if member.namespace == NS_GML && member.local_name == "surfaceMember" {
                                            if let Some(href) = SubtreeReader::xlink_href(&member) {
                                                if let Some(p) = shell_sub.resolve_polygon_href(href) {
                                                    solid.exterior_shell.push(p);
                                                }
                                                shell_sub.skip_element()?;
                                            } else {
                                                let mut member_sub = shell_sub.subtree();
                                                while let Some(surf_info) = member_sub.next_element()? {
                                                    let mut polys = parse_surface_element(&mut member_sub, &surf_info)?;
                                                    solid.exterior_shell.append(&mut polys);
                                                }
                                            }
                                        } else {
                                            shell_sub.skip_element()?;
                                        }
                                    }
                                }
                                _ => {
                                    ext_sub.skip_element()?;
                                }
                            }
                        } else {
                            ext_sub.skip_element()?;
                        }
                    }
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        } else {
            sub.skip_element()?;
        }
    }
    Ok(solid)
}

/// Parse a gml:TriangulatedSurface element.
pub fn parse_triangulated_surface(reader: &mut SubtreeReader<'_>) -> Result<TriangulatedSurface, ReaderError> {
    let mut ts = TriangulatedSurface::default();
    let mut sub = reader.subtree();
    while let Some(info) = sub.next_element()? {
        if info.namespace == NS_GML {
            match info.local_name.as_str() {
                "trianglePatches" | "patches" => {
                    let mut patches_sub = sub.subtree();
                    while let Some(patch_info) = patches_sub.next_element()? {
                        if patch_info.namespace == NS_GML && patch_info.local_name == "Triangle" {
                            // Triangle is structured like a Polygon
                            ts.triangles.push(parse_polygon(&mut patches_sub)?);
                        } else {
                            patches_sub.skip_element()?;
                        }
                    }
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        } else {
            sub.skip_element()?;
        }
    }
    Ok(ts)
}

/// Parse a gml:Point element.
pub fn parse_point(reader: &mut SubtreeReader<'_>) -> Result<DirectPosition, ReaderError> {
    let mut pos = DirectPosition::default();
    let mut sub = reader.subtree();
    while let Some(info) = sub.next_element()? {
        if info.namespace == NS_GML {
            match info.local_name.as_str() {
                "pos" => {
                    let text = sub.read_text()?;
                    pos = parse_pos(&text);
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        } else {
            sub.skip_element()?;
        }
    }
    Ok(pos)
}

/// Parse a gml:MultiCurve element.
pub fn parse_multi_curve(reader: &mut SubtreeReader<'_>) -> Result<MultiCurve, ReaderError> {
    let mut mc = MultiCurve::default();
    let mut sub = reader.subtree();
    while let Some(info) = sub.next_element()? {
        if info.namespace == NS_GML {
            match info.local_name.as_str() {
                "curveMember" => {
                    let mut member_sub = sub.subtree();
                    while let Some(curve_info) = member_sub.next_element()? {
                        if curve_info.namespace == NS_GML && curve_info.local_name == "LineString" {
                            let mut ls_sub = member_sub.subtree();
                            while let Some(ls_child) = ls_sub.next_element()? {
                                if ls_child.namespace == NS_GML && ls_child.local_name == "posList" {
                                    let text = ls_sub.read_text()?;
                                    mc.curves.push(parse_pos_list(&text));
                                } else {
                                    ls_sub.skip_element()?;
                                }
                            }
                        } else {
                            member_sub.skip_element()?;
                        }
                    }
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        } else {
            sub.skip_element()?;
        }
    }
    Ok(mc)
}

/// Dispatch geometry parsing based on element name.
/// Call this after entering a geometry wrapper element (e.g. lod2Solid)
/// where the child is a GML geometry element.
pub fn parse_geometry_child(reader: &mut SubtreeReader<'_>) -> Result<GeometryResult, ReaderError> {
    let mut sub = reader.subtree();
    while let Some(info) = sub.next_element()? {
        if info.namespace == NS_GML {
            let gml_id = SubtreeReader::gml_id(&info);
            match info.local_name.as_str() {
                "MultiSurface" => {
                    let mut ms = parse_multi_surface(&mut sub)?;
                    ms.gml_id = gml_id;
                    return Ok(GeometryResult::MultiSurface(ms));
                }
                "Solid" => {
                    let mut s = parse_solid(&mut sub)?;
                    s.gml_id = gml_id;
                    return Ok(GeometryResult::Solid(s));
                }
                "Polygon" => {
                    let mut p = parse_polygon(&mut sub)?;
                    p.gml_id = gml_id;
                    return Ok(GeometryResult::Polygon(p));
                }
                "TriangulatedSurface" | "Tin" => {
                    let mut t = parse_triangulated_surface(&mut sub)?;
                    t.gml_id = gml_id;
                    return Ok(GeometryResult::TriangulatedSurface(t));
                }
                "Point" => {
                    let p = parse_point(&mut sub)?;
                    return Ok(GeometryResult::Point(p));
                }
                "MultiCurve" => {
                    let mut mc = parse_multi_curve(&mut sub)?;
                    mc.gml_id = gml_id;
                    return Ok(GeometryResult::MultiCurve(mc));
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        } else {
            sub.skip_element()?;
        }
    }
    Err(ReaderError::UnexpectedStructure("Expected a GML geometry element".into()))
}

/// Result of parsing a geometry child element.
#[derive(Debug, Clone)]
pub enum GeometryResult {
    MultiSurface(MultiSurface),
    Solid(Solid),
    Polygon(Polygon),
    TriangulatedSurface(TriangulatedSurface),
    Point(DirectPosition),
    MultiCurve(MultiCurve),
}
