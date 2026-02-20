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
}
