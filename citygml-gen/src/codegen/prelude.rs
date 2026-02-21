use std::path::Path;

use crate::error::GenError;

/// Write a lib.rs file with citygml-core re-exports, `pub mod` declarations,
/// and `pub use` re-exports for cross-module visibility.
pub fn write_lib_file(
    src_dir: &Path,
    module_names: &[String],
    dry_run: bool,
    verbose: bool,
) -> Result<(), GenError> {
    let mut content = String::from("#![allow(unused_imports)]\n\n");

    // Re-export citygml-core modules so generated `crate::` paths resolve
    content.push_str("pub use citygml_core::error;\n");
    content.push_str("pub use citygml_core::geometry;\n");
    content.push_str("pub use citygml_core::namespace;\n");
    content.push_str("pub use citygml_core::from_gml;\n");
    content.push_str("pub use citygml_core::primitives;\n");
    content.push_str("pub use citygml_core::gml_reader;\n");
    content.push_str("pub use citygml_core::gml_geometry;\n");

    content.push('\n');

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
            eprintln!("  [dry-run] Would write lib.rs ({} modules)", module_names.len());
        }
    } else {
        let path = src_dir.join("lib.rs");
        if verbose {
            eprintln!("  Writing lib.rs...");
        }
        std::fs::write(path, content)?;
    }

    Ok(())
}
