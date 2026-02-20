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

    // Phase 2+3: Resolve to IR
    if args.verbose {
        eprintln!("\nPhase 2: Resolving types...");
    }
    let model = resolve::resolver::build_model(&raw_model, args.verbose);

    if args.verbose {
        eprintln!("  Resolved classes:     {}", model.classes.len());
        eprintln!("  Resolved enumerations:{}", model.enumerations.len());
        eprintln!("  Resolved data types:  {}", model.data_types.len());
        eprintln!("  Sorted class order:   {} entries", model.sorted_class_ids.len());

        // Count total own properties
        let total_props: usize = model.classes.values().map(|c| c.own_properties.len()).sum();
        eprintln!("  Total own properties: {total_props}");

        // Show abstract vs concrete
        let abstract_count = model.classes.values().filter(|c| c.is_abstract).count();
        let concrete_count = model.classes.values().filter(|c| !c.is_abstract).count();
        eprintln!("  Abstract classes:     {abstract_count}");
        eprintln!("  Concrete classes:     {concrete_count}");
    }

    // Phase 4: Code generation
    if args.verbose {
        eprintln!("\nPhase 3: Generating code...");
    }

    if !args.dry_run {
        std::fs::create_dir_all(&args.output)
            .with_context(|| format!("Failed to create output directory: {}", args.output.display()))?;
    }

    let module_names =
        codegen::module_writer::generate_all(&model, &args.output, args.dry_run, args.verbose)?;

    // Write mod.rs
    codegen::prelude::write_mod_file(&args.output, &module_names, args.dry_run, args.verbose)?;

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
