# CLAUDE.md

## Project Overview

Cargo workspace with two crates that parse the CityGML 3.0 UML model (XMI) and generate Rust types + a GML reader:

- **`citygml-gen`** — Binary. Parses CityGML 3.0 XMI from Enterprise Architect, generates Rust code.
- **`citygml-io`** — Library. Contains generated types, geometry stubs, and a GML 3.0 file reader.

## Quick Commands

```bash
# Generate types + reader from XMI
cargo run -p citygml-gen -- \
  --input "path/to/CityGML_3.0.xml" \
  --output citygml-io/src/generated/ \
  --with-reader

# Build everything
cargo build

# Run all tests
cargo test -p citygml-io

# Run specific test with output
cargo test -p citygml-io --test read_building -- --nocapture
```

## Architecture

### citygml-gen Pipeline

1. **XMI Parsing** (`src/xmi/`) — State-machine parser using quick-xml. Handles windows-1252 encoding and self-closing elements (`Event::Empty`).
2. **Type Resolution** (`src/resolve/`) — Converts raw XMI data into `UmlModel` with resolved type references, ancestor chains, and topological sort.
3. **Code Generation** (`src/codegen/`) — Uses proc-macro2/quote to emit Rust code, formatted with prettyplease. One `.rs` file per CityGML package.

### citygml-io Structure

- `gml_reader.rs` — Namespace-aware XML cursor wrapping quick-xml. `SubtreeReader` scopes parsing to element children.
- `gml_geometry.rs` — Hand-written parsers for GML 3.2 geometry (Polygon, MultiSurface, Solid, etc.).
- `from_gml.rs` + `primitives.rs` — `FromGml` trait + impls for primitives and geometry types.
- `city_model.rs` — Top-level `CitygmlReader::from_path()` API.
- `generated/` — 17 modules + `dispatchers.rs`, all auto-generated. **Do not edit by hand.**

## Key Conventions

- **Rust 2024 edition** — No `ref mut` in pattern matching. Use `option.as_mut()` instead.
- **Property namespace resolution** — Each property's XML namespace comes from its defining ancestor's package. GML-inherited properties (`name`, `description`, `identifier`, `boundedBy`) use `gml:` namespace.
- **Property deduplication** — When inherited properties collide by snake_case name, first/ancestor occurrence wins.
- **Abstract types** — Use enum dispatch: each abstract class generates a trait (with `Trait` suffix, e.g. `AbstractCityObjectTrait`) + an enum (plain name, e.g. `AbstractCityObject`) with one variant per concrete descendant. Required abstract fields are promoted to `Option<EnumType>` since enums can't implement `Default` meaningfully.
- **Enum boxing** — Enum variants are boxed when the abstract class has >8 concrete descendants to control stack size.
- **ADEOf* types** — Removed entirely (114 types, 0 implementations). Fields referencing them are skipped.
- **All structs derive Clone** — No more `Box<dyn Trait>` fields anywhere.
- **Self-closing XML** — `Event::Empty` elements require `pop_empty_if_needed()` before reading the next event to keep depth tracking correct.

## Code Generation Notes

- Generated files have `#![allow(unused_imports, unused_mut, unused_variables)]` — this is intentional.
- Re-running the generator overwrites `citygml-io/src/generated/`. The hand-written framework files in `citygml-io/src/` are not affected.
- CLI flags: `--verbose` for progress, `--dry-run` to skip writing, `--packages core,building` to filter.

## Test Fixtures

Located in `citygml-io/tests/fixtures/`. Downloaded from the OGC CityGML-3.0Encodings GitHub repo. Two files:
- `Building_CityGML3.0_LOD2_with_several_attributes.gml` — LOD2 building with boundaries, solid (xlink:href), height, function, roofType, address.
- `BuildingUnits_Storeys_xlink.gml` — Building with units and xlink references.

## Known Limitations

- xlink:href references in geometry (e.g., Solid with `<gml:surfaceMember xlink:href="#..."/>`) are not resolved — the referenced polygons are skipped.
- `ParsedCityModel` currently only dispatches Building, BuildingPart, and ReliefFeature. Other feature types are silently skipped.
- 1 unresolved external type: `grid` attribute in `RasterRelief`.
