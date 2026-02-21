/// Direct position (a point in coordinate space).
#[derive(Debug, Clone, Default)]
pub struct DirectPosition {
    pub coordinates: Vec<f64>,
}

/// A closed ring of coordinates defined by a flat list of ordinates.
#[derive(Debug, Clone, Default)]
pub struct LinearRing {
    pub pos_list: Vec<f64>,
}

/// A polygon with an exterior ring and zero or more interior rings (holes).
#[derive(Debug, Clone, Default)]
pub struct Polygon {
    pub exterior: Option<LinearRing>,
    pub interior: Vec<LinearRing>,
    pub gml_id: Option<String>,
}

/// A collection of surface (polygon) members.
#[derive(Debug, Clone, Default)]
pub struct MultiSurface {
    pub surface_members: Vec<Polygon>,
    pub gml_id: Option<String>,
}

/// A solid geometry with an exterior shell of polygons.
#[derive(Debug, Clone, Default)]
pub struct Solid {
    pub exterior_shell: Vec<Polygon>,
    pub gml_id: Option<String>,
}

/// A triangulated surface composed of triangle patches.
#[derive(Debug, Clone, Default)]
pub struct TriangulatedSurface {
    pub triangles: Vec<Polygon>,
    pub gml_id: Option<String>,
}

/// A multi-curve stub (flat list of coordinate lists).
#[derive(Debug, Clone, Default)]
pub struct MultiCurve {
    pub curves: Vec<Vec<f64>>,
    pub gml_id: Option<String>,
}
