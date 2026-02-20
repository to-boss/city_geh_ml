use std::path::Path;

use crate::error::GenError;

/// Write a mod.rs file with `pub mod` declarations for each generated module.
pub fn write_mod_file(
    output_dir: &Path,
    module_names: &[String],
    dry_run: bool,
    verbose: bool,
) -> Result<(), GenError> {
    let mut content = String::new();
    for name in module_names {
        content.push_str(&format!("pub mod {name};\n"));
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
