use std::path::PathBuf;

use clap::Parser;

/// CityGML 3.0 UML→Rust code generator.
///
/// Parses CityGML 3.0 XMI (UML model exported from Enterprise Architect)
/// and generates Rust types (traits, structs, enums) for every CityGML class.
#[derive(Parser, Debug)]
#[command(name = "citygml-gen", version, about)]
pub struct Args {
    /// Path to the CityGML 3.0 XMI file.
    #[arg(short, long)]
    pub input: PathBuf,

    /// Output directory for generated .rs files.
    #[arg(short, long, default_value = "src/generated")]
    pub output: PathBuf,

    /// Print detailed progress information.
    #[arg(short, long)]
    pub verbose: bool,

    /// Parse and resolve but do not write any files.
    #[arg(long)]
    pub dry_run: bool,

    /// Only generate for the specified packages (comma-separated).
    #[arg(long, value_delimiter = ',')]
    pub packages: Option<Vec<String>>,

    /// Also generate FromGml reader impls to this directory.
    /// Defaults to `<output>/` (same as type output) if flag is present without value.
    #[arg(long)]
    pub with_reader: Option<Option<PathBuf>>,

    /// Dump the resolved IR (UmlModel) to a text file and skip code generation.
    #[arg(long, default_missing_value = "ir_dump.txt", num_args = 0..=1)]
    pub emit_ir: Option<PathBuf>,
}
