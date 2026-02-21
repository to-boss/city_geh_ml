# CityGML 3.0 Rust Toolkit

A Rust workspace for working with [OGC CityGML 3.0](https://www.ogc.org/standard/citygml/) data. Parses the official UML model (XMI) to generate strongly-typed Rust structs, traits, and enums covering all 17 CityGML packages, then provides a GML reader to deserialize `.gml` files into those types.

## Crates

| Crate | Type | Description |
|-------|------|-------------|
| `citygml-gen` | Binary | Code generator: XMI &rarr; Rust types + FromGml reader impls |
| `citygml-io` | Library | Generated types, geometry types, and GML file reader |

## Usage

### Reading a CityGML 3.0 file

```rust
use citygml_io::city_model::CitygmlReader;
use citygml_io::BoundaryAccessors;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = CitygmlReader::from_path("city.gml")?;

    for building in &model.buildings {
        println!("ID: {}", building.feature_id.0);
        println!("Name: {:?}", building.name);
        println!("Function: {:?}", building.function);
        println!("Boundaries: {}", building.boundary.len());

        for wall in building.boundary.wall_surfaces() {
            println!("  Wall lod2MultiSurface: {:?}", wall.lod2_multi_surface);
        }

        if let Some(solid) = &building.lod2_solid {
            println!("LOD2 Solid shells: {}", solid.exterior_shell.len());
        }
    }

    Ok(())
}
```

### Regenerating types from XMI

The generated code lives in `citygml-io/src/generated/`. To regenerate from the CityGML 3.0 UML model:

```bash
cargo run -p citygml-gen -- \
  --input path/to/CityGML_3.0.xml \
  --output citygml-io/src/generated/ \
  --with-reader
```

The XMI file is the CityGML 3.0 UML model exported from Enterprise Architect. It can be obtained from the [OGC CityGML 3.0 Conceptual Model](https://github.com/opengeospatial/CityGML-3.0CM).

#### Generator options

| Flag | Description |
|------|-------------|
| `--input <PATH>` | Path to the CityGML 3.0 XMI file (required) |
| `--output <DIR>` | Output directory for generated `.rs` files (default: `src/generated`) |
| `--with-reader [DIR]` | Also generate `FromGml` deserialization impls (optionally into a separate directory) |
| `--packages core,building` | Only generate specific packages (comma-separated) |
| `--emit-ir [FILE]` | Dump the resolved IR (`UmlModel`) to a text file and skip code generation (default: `ir_dump.txt`) |
| `--verbose` | Print detailed progress |
| `--dry-run` | Parse and resolve without writing files |

## Generated Code

The generator produces Rust code for all 268 classes across 17 CityGML packages:

- **34 abstract classes** &rarr; traits (with `Trait` suffix) + enum dispatch types with one variant per concrete descendant
- **234 concrete classes** &rarr; structs implementing the appropriate traits, all deriving `Clone`
- **13 enums** &rarr; Rust enums with `FromGml` deserialization
- **142 data types** &rarr; codelist newtypes (`struct FooValue(pub String)`) and structured data types
- **114 ADEOf\* extension types** &rarr; removed (0 implementations exist)

### Enum Dispatch

Abstract type hierarchies use enum dispatch instead of trait objects. Since the full CityGML type hierarchy is known at compile time, each abstract class generates an enum whose variants wrap the concrete descendants:

```rust
// Trait for accessor methods
pub trait AbstractSpaceBoundaryTrait: AbstractCityObjectTrait {}

// Enum with one variant per concrete subtype (boxed when >8 variants)
#[derive(Debug, Clone)]
pub enum AbstractSpaceBoundary {
    CeilingSurface(Box<CeilingSurface>),
    GroundSurface(Box<GroundSurface>),
    RoofSurface(Box<RoofSurface>),
    WallSurface(Box<WallSurface>),
    // ...
}
```

This enables `Clone`, `Default`, and pattern matching on all types &mdash; no `Box<dyn Trait>` anywhere.

### Packages

Appearance, Bridge, Building, CityFurniture, CityObjectGroup, Construction, Core, Dynamizer, Generics, LandUse, PointCloud, Relief, Transportation, Tunnel, Vegetation, Versioning, WaterBody

### Example generated types

```rust
// Codelist newtype
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct BuildingFunctionValue(pub String);

// Concrete class with inherited + own properties
#[derive(Debug, Clone, Default)]
pub struct Building {
    pub feature_id: ID,
    pub name: Vec<String>,
    pub boundary: Vec<AbstractSpaceBoundary>,
    pub lod2_solid: Option<Solid>,
    pub function: Vec<BuildingFunctionValue>,
    pub roof_type: Option<RoofTypeValue>,
    pub height: Vec<Height>,
    // ... 40+ fields from inheritance chain
}
```

## Architecture

```
                 CityGML_3.0.xml (XMI)
                         |
                    citygml-gen
                    /    |    \
            XMI Parse  Resolve  Codegen
            (quick-xml) (topo-  (proc-macro2
             + encoding  sort)   + quote)
                         |
              citygml-io/src/generated/
              (17 modules + dispatchers)
                         |
                    citygml-io
                    /    |    \
            GmlReader  FromGml  Geometry
            (ns-aware   trait   (Polygon,
             XML cursor)        Solid, ...)
                         |
                  CitygmlReader
                  ::from_path()
```

### Why an Intermediate Representation?

The generator doesn't go straight from XMI to Rust. Enterprise Architect's XMI export is a flat soup of elements linked by opaque `xmi:id` strings &mdash; a raw class just has `generalizations: ["EAID_ABC123"]` and attribute types like `type_idref: "EAID_DEF456"`. You can't generate code from this because you don't know what anything refers to.

The **resolve** step (`Raw` &rarr; `UmlModel` IR) does three things:

1. **Type resolution** &mdash; Converts opaque ID strings into a typed `UmlTypeRef` enum (`Known(id)`, `External(GmPoint)`, `Unresolved(...)`) by cross-referencing classes, enums, data types, EA stubs, and geometry connector metadata.
2. **Topological sorting** &mdash; Parents must be defined before children in the output. The raw XMI gives them in arbitrary document order. The resolve stage does a DAG sort on the inheritance graph.
3. **Semantic queries** &mdash; The `UmlModel` provides computed queries the raw data can't answer: `ancestor_chain()`, `all_properties()` (inherited + own, deduplicated), `concrete_descendants()` (for enum dispatch), `should_skip_prop()` (filters ADEOf\* dead ends), and `non_cloneable_ids()` (transitive Clone analysis).

The raw XMI knows what's *in the file*. The IR knows what it *means*. The codegen only needs to care about *how to emit Rust*.

### GML Reader Design

The reader uses a namespace-aware XML cursor (`GmlReader` / `SubtreeReader`) built on top of `quick-xml`. Parsing follows a recursive descent pattern:

1. **`CitygmlReader::from_path()`** opens the file and iterates `<cityObjectMember>` children
2. Each member is dispatched by `(namespace, localName)` to the appropriate `FromGml::from_gml()` impl
3. Generated `FromGml` impls match child elements by namespace+name and delegate to field-level parsers
4. Geometry elements are parsed by hand-written parsers in `gml_geometry.rs`
5. Abstract types use generated dispatcher functions that return enum dispatch types matching all known concrete subtypes

## Building

```bash
# Build everything
cargo build

# Run tests
cargo test -p citygml-io

# Run with output
cargo test -p citygml-io -- --nocapture
```

Requires Rust 2024 edition (nightly or stable 1.85+).

## Test Fixtures

Integration tests use CityGML 3.0 example files from the [OGC CityGML-3.0Encodings](https://github.com/opengeospatial/CityGML-3.0Encodings) repository, stored in `citygml-io/tests/fixtures/`.

## Known Limitations

- **Limited xlink resolution**: Geometry-level `xlink:href` references (e.g., `<gml:surfaceMember xlink:href="#p1"/>`) are resolved via a polygon registry (single-pass, forward-defined polygons only). Feature-level xlink references are not supported.
- **Partial feature dispatch**: `ParsedCityModel` currently collects Building, BuildingPart, and ReliefFeature. Other CityGML feature types are parsed but not yet collected.
- **No CRS handling**: Coordinate reference systems are not interpreted. Coordinates are stored as raw `f64` values.
- **One unresolved external type**: The `grid` attribute in `RasterRelief` references an external grid coverage type that is not modeled.

## Future Plans

- [**Standalone generated crate**](docs/standalone-crate-plan.md) — Restructure the workspace so the code generator produces a complete, self-contained `citygml-types` crate instead of generating into `citygml-io/src/generated/`. Splits the workspace into `citygml-core` (hand-written framework), `citygml-types` (fully generated), and a slimmed `citygml-io` wrapper.
- [**XSD-based code generation**](docs/xsd-migration-plan.md) — Replace the XMI (Enterprise Architect UML export) parser with an XSD parser that reads the official OGC XML Schema files directly. This resolves the unresolved `grid` type, eliminates EA lock-in, and aligns generated element names exactly with GML encoding.

## License

This project is not yet licensed. The CityGML 3.0 conceptual model is an OGC standard.
