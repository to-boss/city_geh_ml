#![allow(dead_code)]

mod cli;
mod codegen;
mod error;
mod ir;
mod resolve;
mod util;
mod xmi;

use anyhow::{Context, Result};
use clap::Parser;

use cli::Args;

fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        eprintln!("citygml-gen — CityGML 3.0 Rust code generator");
        eprintln!("  Input:  {}", args.input.display());
        eprintln!("  Output: {}", args.output.display());
        eprintln!("  Crate:  {}", args.crate_name);
        if args.dry_run {
            eprintln!("  Mode:   dry-run");
        }
    }

    // Phase 1: Parse XMI
    if args.verbose {
        eprintln!("\nPhase 1: Parsing XMI...");
    }
    let raw_model = xmi::parser::parse_xmi(&args.input)
        .with_context(|| format!("Failed to parse XMI file: {}", args.input.display()))?;

    if args.verbose {
        eprintln!("  Packages:    {}", raw_model.packages.len());
        eprintln!("  Classes:     {}", raw_model.classes.len());
        eprintln!("  Enumerations:{}", raw_model.enumerations.len());
        eprintln!("  DataTypes:   {}", raw_model.data_types.len());
        eprintln!("  EAStubs:     {}", raw_model.ea_stubs.len());
        eprintln!("  GeometryRefs:{}", raw_model.geometry_refs.len());
    }

    // Phase 2: Resolve to IR
    if args.verbose {
        eprintln!("\nPhase 2: Resolving types...");
    }
    let model = resolve::resolver::build_model(&raw_model, args.verbose);

    if args.verbose {
        eprintln!("  Resolved classes:     {}", model.classes.len());
        eprintln!("  Resolved enumerations:{}", model.enumerations.len());
        eprintln!("  Resolved data types:  {}", model.data_types.len());
        eprintln!("  Sorted class order:   {} entries", model.sorted_class_ids.len());

        let total_props: usize = model.classes.values().map(|c| c.own_properties.len()).sum();
        eprintln!("  Total own properties: {total_props}");

        let abstract_count = model.classes.values().filter(|c| c.is_abstract).count();
        let concrete_count = model.classes.values().filter(|c| !c.is_abstract).count();
        eprintln!("  Abstract classes:     {abstract_count}");
        eprintln!("  Concrete classes:     {concrete_count}");
    }

    // Emit IR dump and exit early if requested
    if let Some(ir_path) = &args.emit_ir {
        std::fs::write(ir_path, format!("{:#?}", model))?;
        println!("IR dumped to {}", ir_path.display());
        return Ok(());
    }

    // Phase 3: Code generation
    if args.verbose {
        eprintln!("\nPhase 3: Generating code + reader impls...");
    }

    let src_dir = args.output.join("src");

    if !args.dry_run {
        std::fs::create_dir_all(&src_dir)
            .with_context(|| format!("Failed to create output directory: {}", src_dir.display()))?;
    }

    let module_names = codegen::module_writer::generate_all(
        &model,
        &src_dir,
        true, // always generate reader impls
        args.dry_run,
        args.verbose,
    )?;

    // Write lib.rs
    codegen::prelude::write_lib_file(&src_dir, &module_names, args.dry_run, args.verbose)?;

    // Write Cargo.toml for the generated crate
    if !args.dry_run {
        let cargo_toml = format!(
            "[package]\n\
             name = \"{}\"\n\
             version = \"0.1.0\"\n\
             edition = \"2024\"\n\
             \n\
             [dependencies]\n\
             citygml-core = {{ path = \"../citygml-core\" }}\n",
            args.crate_name
        );
        let cargo_path = args.output.join("Cargo.toml");
        std::fs::write(&cargo_path, cargo_toml)?;
        if args.verbose {
            eprintln!("  Writing Cargo.toml...");
        }
    }

    if args.verbose {
        eprintln!("\nDone. Generated {} modules.", module_names.len());
    } else if !args.dry_run {
        println!(
            "Generated {} modules in {}",
            module_names.len(),
            args.output.display()
        );
    } else {
        println!(
            "Dry run complete. Would generate {} modules.",
            module_names.len()
        );
    }

    Ok(())
}
