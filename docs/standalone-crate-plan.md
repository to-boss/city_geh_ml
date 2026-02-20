# Plan: Generate a Standalone Crate Instead of Into an Existing One

## Context

Currently `citygml-gen` generates Rust code into `citygml-io/src/generated/`, mixing auto-generated and hand-written code in one crate. This is fragile — re-running the generator overwrites files inside a hand-maintained crate. The goal is to have the generator produce a complete, self-contained crate that other crates simply depend on.

## Target Architecture: 3 Crates

```
citygml-core   (NEW)  — hand-written framework: traits, reader, geometry, namespaces, errors
citygml-types  (NEW)  — fully generated: all CityGML types + FromGml impls + dispatchers
citygml-io     (SLIM) — city_model.rs entry point + re-exports from both crates
citygml-gen    (MOD)  — updated to produce a complete crate at citygml-types/
```

The generated crate (`citygml-types`) depends on `citygml-core` and re-exports it for convenience. `citygml-io` becomes a thin wrapper providing `CitygmlReader` and re-exporting everything so existing test imports keep working.

## Step 1: Create `citygml-core` crate

Create `citygml-core/` with these files moved from `citygml-io/src/`:

| Source | Destination |
|---|---|
| `citygml-io/src/error.rs` | `citygml-core/src/error.rs` |
| `citygml-io/src/from_gml.rs` | `citygml-core/src/from_gml.rs` |
| `citygml-io/src/geometry.rs` | `citygml-core/src/geometry.rs` |
| `citygml-io/src/gml_reader.rs` | `citygml-core/src/gml_reader.rs` |
| `citygml-io/src/gml_geometry.rs` | `citygml-core/src/gml_geometry.rs` |
| `citygml-io/src/primitives.rs` | `citygml-core/src/primitives.rs` |
| `citygml-io/src/namespace.rs` | `citygml-core/src/namespace.rs` |

New files:
- `citygml-core/Cargo.toml` — deps: `quick-xml` (with encoding), `thiserror`
- `citygml-core/src/lib.rs` — module declarations for the 7 modules above

No code changes needed inside these files — all `crate::` references are within the same crate.

## Step 2: Update code generator to produce a full crate

### 2a. Add `CodegenContext` struct

Thread a `core_crate` token through all codegen functions so generated code references `citygml_core::` instead of `crate::`.

```rust
// citygml-gen/src/codegen/mod.rs
pub struct CodegenContext {
    pub core_crate: syn::Path,  // e.g. citygml_core
}
```

### 2b. Change all `crate::` references in generated output

Files to modify in `citygml-gen/src/codegen/`:

- **`external_types.rs`** — `external_type_tokens()`: change 7 occurrences of `crate::geometry::*` to `citygml_core::geometry::*`
- **`mapper.rs`** — `type_ref_to_tokens()`, `prop_to_field_type()`: pass context to `external_type_tokens`
- **`from_gml_gen.rs`** — change all `quote!` paths:
  - `crate::namespace::NS_*` → `citygml_core::namespace::NS_*` (18 namespace consts)
  - `crate::from_gml::FromGml` → `citygml_core::from_gml::FromGml`
  - `crate::gml_reader::{SubtreeReader, ElementInfo}` → `citygml_core::gml_reader::*`
  - `crate::error::ReaderError` → `citygml_core::error::ReaderError`
  - `crate::gml_geometry::parse_*` → `citygml_core::gml_geometry::parse_*`
- **`struct_gen.rs`**, **`trait_gen.rs`**, **`enum_gen_dispatch.rs`**, **`datatype_gen.rs`** — pass `&CodegenContext` through to mapper/external_types calls

### 2c. Generate `Cargo.toml`

Add to `module_writer.rs`: after writing `.rs` files, write `Cargo.toml` at the crate root:

```toml
[package]
name = "citygml-types"
version = "0.1.0"
edition = "2024"

[dependencies]
citygml-core = { path = "../citygml-core" }
```

### 2d. Generate `lib.rs` instead of `mod.rs`

Update `prelude.rs`: rename function, write to `src/lib.rs`, add `pub use citygml_core;` re-export at the top.

### 2e. Update CLI and `main.rs`

- Change `--output` default from `src/generated` to `citygml-types`; semantics now = crate root (writes `<output>/Cargo.toml` + `<output>/src/*.rs`)
- Add `--crate-name` flag (default: `citygml-types`)
- Add `--core-crate` flag (default: `citygml-core`)
- Update `main.rs` to create `<output>/src/` directory, pass context to `generate_all`

## Step 3: Run the generator

```bash
cargo run -p citygml-gen -- \
  --input "path/to/CityGML_3.0.xml" \
  --output citygml-types \
  --with-reader
```

This produces a complete `citygml-types/` crate. Add it to workspace members.

## Step 4: Slim down `citygml-io`

- Delete the 7 framework files (moved to `citygml-core`)
- Delete `src/generated/` directory entirely
- Update `Cargo.toml`: replace `quick-xml`/`thiserror`/`anyhow` with deps on `citygml-core` + `citygml-types`
- Rewrite `lib.rs`:

```rust
pub use citygml_core::{error, from_gml, geometry, gml_geometry, gml_reader, namespace, primitives};
pub use citygml_types::*;
pub mod city_model;
```

- Update `city_model.rs` imports: `crate::generated::*` → `citygml_types::*`, `crate::error`/etc → `citygml_core::*`

## Step 5: Update workspace `Cargo.toml`

```toml
[workspace]
members = ["citygml-gen", "citygml-core", "citygml-types", "citygml-io"]
resolver = "3"
```

## Step 6: Update docs

Update `CLAUDE.md` architecture section and quick commands to reflect new crate structure.

## Verification

1. `cargo build` — all 4 crates compile
2. `cargo test -p citygml-io` — `read_building` and `geometry` tests pass (imports work through re-exports)
3. Re-run generator → `citygml-types/` cleanly overwritten, still compiles
4. Spot-check a generated file to confirm `citygml_core::` paths (not `crate::`)
