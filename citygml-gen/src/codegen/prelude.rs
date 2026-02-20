use std::path::Path;

use crate::error::GenError;

/// Write a mod.rs file with `pub mod` declarations and `pub use` re-exports.
pub fn write_mod_file(
    output_dir: &Path,
    module_names: &[String],
    dry_run: bool,
    verbose: bool,
) -> Result<(), GenError> {
    let mut content = String::from("#![allow(unused_imports)]\n\n");

    // Module declarations
    for name in module_names {
        content.push_str(&format!("pub mod {name};\n"));
    }

    content.push('\n');

    // Re-export all types for cross-module visibility
    for name in module_names {
        if name != "dispatchers" {
            content.push_str(&format!("pub use {name}::*;\n"));
        }
    }

    if dry_run {
        if verbose {
            eprintln!("  [dry-run] Would write mod.rs ({} modules)", module_names.len());
        }
    } else {
        let path = output_dir.join("mod.rs");
        if verbose {
            eprintln!("  Writing mod.rs...");
        }
        std::fs::write(path, content)?;
    }

    Ok(())
}
