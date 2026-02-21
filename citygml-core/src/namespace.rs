/// GML 3.2 namespace
pub const NS_GML: &str = "http://www.opengis.net/gml/3.2";

/// XLink namespace
pub const NS_XLINK: &str = "http://www.w3.org/1999/xlink";

/// XML Schema Instance namespace
pub const NS_XSI: &str = "http://www.w3.org/2001/XMLSchema-instance";

// CityGML 3.0 module namespaces

pub const NS_CORE: &str = "http://www.opengis.net/citygml/3.0";
pub const NS_BUILDING: &str = "http://www.opengis.net/citygml/building/3.0";
pub const NS_CONSTRUCTION: &str = "http://www.opengis.net/citygml/construction/3.0";
pub const NS_BRIDGE: &str = "http://www.opengis.net/citygml/bridge/3.0";
pub const NS_TUNNEL: &str = "http://www.opengis.net/citygml/tunnel/3.0";
pub const NS_TRANSPORTATION: &str = "http://www.opengis.net/citygml/transportation/3.0";
pub const NS_VEGETATION: &str = "http://www.opengis.net/citygml/vegetation/3.0";
pub const NS_WATER_BODY: &str = "http://www.opengis.net/citygml/waterbody/3.0";
pub const NS_LAND_USE: &str = "http://www.opengis.net/citygml/landuse/3.0";
pub const NS_RELIEF: &str = "http://www.opengis.net/citygml/relief/3.0";
pub const NS_CITY_FURNITURE: &str = "http://www.opengis.net/citygml/cityfurniture/3.0";
pub const NS_GENERICS: &str = "http://www.opengis.net/citygml/generics/3.0";
pub const NS_CITY_OBJECT_GROUP: &str = "http://www.opengis.net/citygml/cityobjectgroup/3.0";
pub const NS_APPEARANCE: &str = "http://www.opengis.net/citygml/appearance/3.0";
pub const NS_POINT_CLOUD: &str = "http://www.opengis.net/citygml/pointcloud/3.0";
pub const NS_DYNAMIZER: &str = "http://www.opengis.net/citygml/dynamizer/3.0";
pub const NS_VERSIONING: &str = "http://www.opengis.net/citygml/versioning/3.0";

/// Map a CityGML package name (from the UML model) to its XML namespace URI.
pub fn package_to_namespace(package_name: &str) -> Option<&'static str> {
    match package_name {
        "Core" => Some(NS_CORE),
        "Building" => Some(NS_BUILDING),
        "Construction" => Some(NS_CONSTRUCTION),
        "Bridge" => Some(NS_BRIDGE),
        "Tunnel" => Some(NS_TUNNEL),
        "Transportation" => Some(NS_TRANSPORTATION),
        "Vegetation" => Some(NS_VEGETATION),
        "WaterBody" => Some(NS_WATER_BODY),
        "LandUse" => Some(NS_LAND_USE),
        "Relief" => Some(NS_RELIEF),
        "CityFurniture" => Some(NS_CITY_FURNITURE),
        "Generics" => Some(NS_GENERICS),
        "CityObjectGroup" => Some(NS_CITY_OBJECT_GROUP),
        "Appearance" => Some(NS_APPEARANCE),
        "PointCloud" => Some(NS_POINT_CLOUD),
        "Dynamizer" => Some(NS_DYNAMIZER),
        "Versioning" => Some(NS_VERSIONING),
        _ => None,
    }
}
