use citygml_io::gml_geometry::*;
use citygml_io::gml_reader::reader_from_bytes;

#[test]
fn test_parse_polygon() {
    let xml = br#"<root xmlns:gml="http://www.opengis.net/gml/3.2">
        <gml:Polygon gml:id="p1">
            <gml:exterior>
                <gml:LinearRing>
                    <gml:posList>0 0 0 10 0 0 10 10 0 0 10 0 0 0 0</gml:posList>
                </gml:LinearRing>
            </gml:exterior>
        </gml:Polygon>
    </root>"#;

    let mut reader = reader_from_bytes(xml);
    let root = reader.next_start_element().unwrap().unwrap();
    assert_eq!(root.local_name, "root");

    let mut sub = reader.subtree();
    let info = sub.next_element().unwrap().unwrap();
    assert_eq!(info.local_name, "Polygon");

    let polygon = parse_polygon(&mut sub).unwrap();
    assert!(polygon.exterior.is_some());
    let ring = polygon.exterior.as_ref().unwrap();
    // 5 points * 3 coords = 15 values
    assert_eq!(ring.pos_list.len(), 15);
    assert_eq!(ring.pos_list[0], 0.0);
    assert_eq!(ring.pos_list[3], 10.0);
}

#[test]
fn test_parse_multi_surface() {
    let xml = br#"<root xmlns:gml="http://www.opengis.net/gml/3.2">
        <gml:MultiSurface gml:id="ms1">
            <gml:surfaceMember>
                <gml:Polygon gml:id="p1">
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList>0 0 0 1 0 0 1 1 0 0 0 0</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                </gml:Polygon>
            </gml:surfaceMember>
            <gml:surfaceMember>
                <gml:Polygon gml:id="p2">
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList>2 2 0 3 2 0 3 3 0 2 2 0</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                </gml:Polygon>
            </gml:surfaceMember>
        </gml:MultiSurface>
    </root>"#;

    let mut reader = reader_from_bytes(xml);
    let _ = reader.next_start_element().unwrap();
    let mut sub = reader.subtree();
    let info = sub.next_element().unwrap().unwrap();
    assert_eq!(info.local_name, "MultiSurface");

    let ms = parse_multi_surface(&mut sub).unwrap();
    assert_eq!(ms.surface_members.len(), 2);
    assert!(ms.surface_members[0].exterior.is_some());
    assert!(ms.surface_members[1].exterior.is_some());
}

#[test]
fn test_parse_solid() {
    let xml = br#"<root xmlns:gml="http://www.opengis.net/gml/3.2">
        <gml:Solid gml:id="s1">
            <gml:exterior>
                <gml:Shell>
                    <gml:surfaceMember>
                        <gml:Polygon gml:id="p1">
                            <gml:exterior>
                                <gml:LinearRing>
                                    <gml:posList>0 0 0 1 0 0 1 1 0 0 0 0</gml:posList>
                                </gml:LinearRing>
                            </gml:exterior>
                        </gml:Polygon>
                    </gml:surfaceMember>
                </gml:Shell>
            </gml:exterior>
        </gml:Solid>
    </root>"#;

    let mut reader = reader_from_bytes(xml);
    let _ = reader.next_start_element().unwrap();
    let mut sub = reader.subtree();
    let info = sub.next_element().unwrap().unwrap();
    assert_eq!(info.local_name, "Solid");

    let solid = parse_solid(&mut sub).unwrap();
    assert!(!solid.exterior_shell.is_empty());
    assert_eq!(solid.exterior_shell.len(), 1);
}

#[test]
fn test_parse_pos_list() {
    let coords = parse_pos_list("1.5 2.5 3.5 4.0 5.0 6.0");
    assert_eq!(coords.len(), 6);
    assert_eq!(coords[0], 1.5);
    assert_eq!(coords[5], 6.0);
}
