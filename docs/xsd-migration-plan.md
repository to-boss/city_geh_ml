# XSD vs XMI for CityGML 3.0 Code Generation — Full Analysis

## Context

**Goal**: Fully support GML 3.0 parsing. The current `citygml-gen` pipeline parses CityGML 3.0 UML exported as XMI from Enterprise Architect, builds an IR (`UmlModel`), and generates Rust types + `FromGml` reader implementations. We're evaluating whether the official OGC XML Schema (XSD) files would be a better source for code generation.

## Complete XSD Schema Inventory

OGC publishes one XSD per CityGML package at `https://schemas.opengis.net/citygml/`. All 17 packages are covered:

| Package | XSD path | Concrete types | Key types |
|---------|----------|----------------|-----------|
| Core | `3.0/core.xsd` | CityModel, Address, ImplicitGeometry, ClosureSurface | AbstractCityObject, AbstractSpace, AbstractSpaceBoundary |
| Building | `building/3.0/building.xsd` | Building, BuildingPart, BuildingRoom, Storey, BuildingUnit, BuildingConstructiveElement, BuildingInstallation, BuildingFurniture | AbstractBuilding, AbstractBuildingSubdivision |
| Construction | `construction/3.0/construction.xsd` | OtherConstruction, Door, Window, WallSurface, RoofSurface, FloorSurface, CeilingSurface, etc. | AbstractConstruction, AbstractConstructionSurface, AbstractFillingSurface |
| Transportation | `transportation/3.0/transportation.xsd` | Road, Railway, Track, Waterway, Square, Section, Intersection, TrafficSpace, TrafficArea, Marking, etc. | AbstractTransportationSpace |
| Bridge | `bridge/3.0/bridge.xsd` | Bridge, BridgePart, BridgeRoom, BridgeConstructiveElement, BridgeInstallation, BridgeFurniture | AbstractBridge |
| Tunnel | `tunnel/3.0/tunnel.xsd` | Tunnel, TunnelPart, HollowSpace, TunnelConstructiveElement, TunnelInstallation, TunnelFurniture | AbstractTunnel |
| Appearance | `appearance/3.0/appearance.xsd` | Appearance, ParameterizedTexture, GeoreferencedTexture, X3DMaterial, TexCoordList, TexCoordGen | AbstractSurfaceData, AbstractTexture |
| Vegetation | `vegetation/3.0/vegetation.xsd` | SolitaryVegetationObject, PlantCover | AbstractVegetationObject |
| WaterBody | `waterbody/3.0/waterBody.xsd` | WaterBody, WaterSurface, WaterGroundSurface | AbstractWaterBoundarySurface |
| Relief | `relief/3.0/relief.xsd` | ReliefFeature, TINRelief, MassPointRelief, BreaklineRelief, RasterRelief | AbstractReliefComponent |
| LandUse | `landuse/3.0/landUse.xsd` | LandUse | — |
| CityFurniture | `cityfurniture/3.0/cityFurniture.xsd` | CityFurniture | — |
| CityObjectGroup | `cityobjectgroup/3.0/cityObjectGroup.xsd` | CityObjectGroup, Role | — |
| Generics | `generics/3.0/generics.xsd` | GenericLogicalSpace, GenericOccupiedSpace, GenericUnoccupiedSpace, GenericThematicSurface + 8 generic attribute types | — |
| Dynamizer | `dynamizer/3.0/dynamizer.xsd` | Dynamizer, GenericTimeseries, StandardFileTimeseries, TabulatedFileTimeseries, CompositeTimeseries, SensorConnection, TimeValuePair, TimeseriesComponent | AbstractTimeseries, AbstractAtomicTimeseries |
| Versioning | `versioning/3.0/versioning.xsd` | Version, VersionTransition, Transaction | — |
| PointCloud | `pointcloud/3.0/pointCloud.xsd` | PointCloud | — |

External dependencies: GML 3.2.1 (`gml/3.2.1/`), OASIS xAL 3.0 (addresses).

---

## Head-to-Head Comparison

| What codegen needs | XMI (current) | XSD |
|---|---|---|
| **Class hierarchy** | `<generalization>` with xmi:idref | `complexContent/extension base=` |
| **Abstract marking** | `isAbstract="true"` on packagedElement | `abstract="true"` on complexType + element |
| **Concrete descendants** | Computed via `concrete_descendants()` | `substitutionGroup` chains (exactly what dispatchers need) |
| **Properties + multiplicity** | `<ownedAttribute>` with `lower`/`upper` | `<element>` with `minOccurs`/`maxOccurs` |
| **XML namespace URIs** | Manual `package_to_namespace()` map | Built into each XSD's `targetNamespace` |
| **XML element names** | Derived from UML class names | Explicit `<element name="...">` declarations |
| **Geometry types** | EA Extension stubs, GM_* name matching (**1 unresolved: `grid`**) | Direct: `type="gml:SolidPropertyType"` etc. **RasterRelief.grid = `gml:RectifiedGridCoverage` — resolved!** |
| **Enumerations** | Explicit UML Enumeration with literals | `simpleType` restrictions with `<enumeration>` values |
| **CodeLists** | Classes with 0 attributes (heuristic) | All are `gml:CodeType` (no distinction between different codelists) |
| **DataType vs Class** | Explicit `uml:DataType` vs `uml:Class` | Both are `complexType` (need heuristic) |
| **ADEOf\* types** | Filter by `ADEOf` prefix | Still present as `adeOf*` elements (same filter) |
| **Associations** | `association_id` + `ownedEnd` | PropertyType wrappers + `gml:AssociationAttributeGroup` |

---

## Verdict: XSD is better suited for full GML parsing

### Why XSD wins for this project

1. **Direct XML encoding** — Element names, namespaces, and property names in the XSD *are* the values in GML files. No UML-to-XML translation layer. This is the single biggest advantage for a GML parser generator.

2. **Solves the unresolved type problem** — RasterRelief's `grid` attribute is `gml:RectifiedGridCoverage` in XSD, explicitly typed. The XMI approach couldn't resolve it.

3. **substitutionGroup = dispatcher routing** — XSD substitutionGroups encode exactly which concrete elements can appear where an abstract element is expected. This is precisely what `dispatchers.rs` generates.

4. **No EA lock-in** — XSD is a W3C standard published by OGC. No dependence on Enterprise Architect's XMI export format.

5. **Fetchable at build time** — All XSDs at well-known OGC URLs. Can be cached locally or vendored.

6. **Consistent patterns across all 17 packages** — Every package follows the same XSD conventions (extension, substitutionGroup, PropertyType wrappers, ADE hooks, CodeType for class/function/usage).

### What XSD loses (and mitigations)

| Lost feature | Impact | Mitigation |
|---|---|---|
| CodeList distinction | All codelists become `gml:CodeType` | Already generate `String` newtypes; no behavior change |
| DataType vs Class | Both are `complexType` | Heuristic: types extending `gml:AbstractFeature*` chain are classes; others are datatypes |
| PropertyType wrapper noise | Association props wrapped in anonymous types | Pattern unwrapping: detect `*PropertyType` suffix or `gml:AbstractFeatureMemberType` base |
| Multi-file parsing | 17 XSD files + GML base | Trivial — parse all, merge into single model |

---

## Implementation Plan

### Decisions
- **Vendor XSD files locally** into `citygml-gen/xsd/` (checked into repo, version-pinned, offline-capable)
- **Replace XMI parser entirely** — delete `src/xmi/`, simplify `src/resolve/`. XSD is strictly better for GML parsing.

### Phase 1: Vendor XSD files

Download all CityGML 3.0 XSDs into `citygml-gen/xsd/`:
```
citygml-gen/xsd/
├── core.xsd
├── building.xsd
├── construction.xsd
├── transportation.xsd
├── bridge.xsd
├── tunnel.xsd
├── appearance.xsd
├── vegetation.xsd
├── waterbody.xsd
├── relief.xsd
├── landuse.xsd
├── cityfurniture.xsd
├── cityobjectgroup.xsd
├── generics.xsd
├── dynamizer.xsd
├── versioning.xsd
└── pointcloud.xsd
```

We do NOT need to vendor GML 3.2 or xAL schemas — we only need to recognize their types (gml:AbstractFeatureType, gml:CodeType, gml:*PropertyType, etc.) as external types, which we can hardcode.

### Phase 2: XSD Raw Parser (`citygml-gen/src/xsd/`)

New module parsing XSD files with quick-xml (already a dependency):

**`src/xsd/raw.rs`** — Raw XSD data structures:
```
RawSchema { target_namespace, imports, complex_types, simple_types, elements }
RawComplexType { name, is_abstract, base_type, own_elements }
RawSimpleType { name, restriction_base, enumeration_values }
RawElement { name, type_ref, substitution_group, is_abstract, min_occurs, max_occurs }
RawProperty { name, type_ref, min_occurs, max_occurs, is_wrapper }
```

**`src/xsd/parser.rs`** — Parse each `.xsd` file into a `RawSchema`. Key parsing targets:
- `<xs:complexType name="..." abstract="true">` with `<xs:complexContent><xs:extension base="...">`
- `<xs:element name="..." type="..." substitutionGroup="...">`
- `<xs:simpleType>` with `<xs:restriction><xs:enumeration>`
- Nested `<xs:element>` within sequences (properties)
- Inline anonymous complexTypes (association wrappers — unwrap to get the referenced element)

**`src/xsd/loader.rs`** — Load all 17 XSDs from `citygml-gen/xsd/`, build a merged `RawXsdModel` with namespace-to-schema mapping. No need to follow `<xs:import schemaLocation>` since we vendor everything.

### Phase 3: XSD to IR Resolution (`src/resolve/`)

Replace `build_model()` with `build_model_from_xsd(raw: &RawXsdModel) -> UmlModel`:

1. **Classify types**: complexType extending gml:Abstract*Type chain becomes `UmlClass`; leaf data complexTypes (Height, Elevation, etc.) become `UmlDataType`; simpleType with enumeration becomes `UmlEnum`
2. **Map properties**: Sequence elements become `UmlProperty`. Type mapping:
   - `gml:CodeType` maps to codelist newtype
   - `gml:SolidPropertyType` maps to `ExternalType::GmSolid`
   - `gml:MultiSurfacePropertyType` maps to `ExternalType::GmMultiSurface`
   - etc. (same external type catalog as current, but mapping from XSD type names instead of EA stub names)
3. **Unwrap association wrappers**: Properties whose type is an inline complexType extending `gml:AbstractFeatureMemberType` — extract the `<xs:element ref="...">` to get the actual referenced type
4. **Build inheritance**: `extension base=` gives parent references
5. **Build concrete descendants**: `substitutionGroup` chains give transitive closure
6. **Topological sort**: Reuse existing `topology.rs`
7. **Assign namespaces**: Each type's namespace = `targetNamespace` of declaring XSD. Each property's namespace = namespace of the type that declares it (not inherits it).
8. **Filter ADEOf\***: Same prefix filter (`adeOf*` elements + `ADEOf*` types)

### Phase 4: Codegen Adjustments

Mostly unchanged, but:
- Remove `package_to_namespace()` from `citygml-gen/src/codegen/mapper.rs` — namespaces now come from XSD data via the IR
- `from_gml_gen.rs` `ns_for_property()` simplified — namespace stored directly on each `UmlProperty`
- Dispatcher element names from XSD `<element name>` — exact match, no UML-to-XML guessing

### Phase 5: CLI Simplification

- Remove `--input` XMI path arg
- Default to vendored XSD directory `citygml-gen/xsd/`
- Optional `--xsd-dir <path>` override for custom XSD sets
- Keep `--output`, `--with-reader`, `--verbose`, `--dry-run`, `--packages` flags

### Phase 6: Cleanup

- Delete `src/xmi/` entirely (parser.rs, raw.rs, reader.rs)
- Delete XMI-specific resolver code from `src/resolve/resolver.rs`
- Update `src/resolve/mod.rs` exports

### Files

| Action | File | Notes |
|--------|------|-------|
| **New** | `citygml-gen/xsd/*.xsd` (17 files) | Vendored OGC schemas |
| **New** | `citygml-gen/src/xsd/mod.rs` | Module declaration |
| **New** | `citygml-gen/src/xsd/raw.rs` | Raw XSD data structures |
| **New** | `citygml-gen/src/xsd/parser.rs` | XSD file parser |
| **New** | `citygml-gen/src/xsd/loader.rs` | Multi-file loader |
| **Rewrite** | `citygml-gen/src/resolve/resolver.rs` | `build_model_from_xsd()` replacing `build_model()` |
| **Modify** | `citygml-gen/src/main.rs` | Use XSD pipeline |
| **Modify** | `citygml-gen/src/cli.rs` | Replace `--input` with `--xsd-dir` |
| **Delete** | `citygml-gen/src/xmi/` | Entire XMI module |
| **Reuse** | `citygml-gen/src/resolve/topology.rs` | Unchanged |
| **Reuse** | `citygml-gen/src/ir/` | Unchanged |
| **Reuse** | `citygml-gen/src/codegen/` | Minor namespace simplification |
| **Unchanged** | `citygml-io/` | Generated code + framework — no changes |

### Verification

1. Vendor all 17 XSD files
2. Build XSD parser, generate types: `cargo run -p citygml-gen -- --output citygml-io/src/generated/ --with-reader`
3. `cargo build -p citygml-io` — compile generated code
4. `cargo test -p citygml-io` — all existing GML reading tests pass
5. Diff generated output against current XMI-generated output (expect near-identical, with improvements for previously-unresolved types)
6. Verify RasterRelief.grid resolves to a real geometry type instead of `()`
7. Verify type counts: ~268 classes, ~13 enums, 17 packages
