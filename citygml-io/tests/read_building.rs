use citygml_io::city_model::CitygmlReader;

#[test]
fn test_read_building_lod2() {
    let model = CitygmlReader::from_path(
        "tests/fixtures/Building_CityGML3.0_LOD2_with_several_attributes.gml",
    )
    .expect("Failed to parse LOD2 building file");

    assert_eq!(model.buildings.len(), 1, "Should have exactly 1 building");
    let b = &model.buildings[0];

    // Check gml:id
    assert_eq!(b.feature_id.0, "DEBY_LOD2_5744682");

    // Check name
    assert_eq!(b.name.len(), 1);
    assert_eq!(b.name[0], "DEBY_LOD2_5744682");

    // Check function (codelist value)
    assert!(!b.function.is_empty(), "Building should have a function");
    assert_eq!(b.function[0].0, "31001_9998");

    // Check roofType (codelist value)
    assert!(b.roof_type.is_some(), "Building should have a roofType");
    assert_eq!(b.roof_type.as_ref().unwrap().0, "3100");

    // Check boundary surfaces
    assert!(
        b.boundary.len() >= 3,
        "Building should have at least 3 boundary surfaces (Roof + Wall + Ground), got {}",
        b.boundary.len()
    );

    // Check lod2Solid geometry
    assert!(
        b.lod2_solid.is_some(),
        "Building should have lod2Solid geometry"
    );
    // Note: this file's Solid uses xlink:href references (not inline polygons),
    // so exterior_shell is empty until xlink resolution is implemented.

    // Check creation date
    assert!(b.creation_date.is_some());
    assert_eq!(b.creation_date.as_ref().unwrap(), "2014-10-08T00:00:00");

    // Check external reference
    assert!(!b.external_reference.is_empty());

    // Check height (from con:height)
    assert!(!b.height.is_empty(), "Building should have height data");
}

#[test]
fn test_read_building_units() {
    let model = CitygmlReader::from_path(
        "tests/fixtures/BuildingUnits_Storeys_xlink.gml",
    )
    .expect("Failed to parse BuildingUnits file");

    // This file should have at least one building
    assert!(
        !model.buildings.is_empty(),
        "Should have at least one building"
    );
}
