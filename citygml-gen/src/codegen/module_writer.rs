use std::path::Path;

use indexmap::IndexMap;
use proc_macro2::TokenStream;

use crate::codegen::datatype_gen::generate_datatype;
use crate::codegen::enum_gen::generate_enum;
use crate::codegen::from_gml_gen;
use crate::codegen::struct_gen::{generate_codelist, generate_struct};
use crate::codegen::trait_gen::generate_trait;
use crate::error::GenError;
use crate::ir::model::UmlModel;
use crate::util::naming::to_snake_case;

/// Per-package collected token streams.
struct PackageTokens {
    module_name: String,
    tokens: Vec<TokenStream>,
}

/// Generate all .rs files from the model.
/// When `with_reader` is true, also generates `FromGml` impls and dispatchers.
pub fn generate_all(
    model: &UmlModel,
    output_dir: &Path,
    with_reader: bool,
    dry_run: bool,
    verbose: bool,
) -> Result<Vec<String>, GenError> {
    // Group items by package
    let mut pkg_tokens: IndexMap<String, PackageTokens> = IndexMap::new();

    // Ensure all packages have entries
    for (pkg_id, pkg) in &model.packages {
        let module_name = to_snake_case(&pkg.name);
        pkg_tokens.entry(pkg_id.clone()).or_insert_with(|| PackageTokens {
            module_name,
            tokens: Vec::new(),
        });
    }

    // ── Type definitions ──

    // Generate enumerations
    for (_id, uml_enum) in &model.enumerations {
        let ts = generate_enum(uml_enum);
        if let Some(pt) = pkg_tokens.get_mut(&uml_enum.package_id) {
            pt.tokens.push(ts);
        }
        // Also generate FromGml for this enum
        if with_reader {
            let reader_ts = from_gml_gen::generate_from_gml_enum(uml_enum);
            if let Some(pt) = pkg_tokens.get_mut(&uml_enum.package_id) {
                pt.tokens.push(reader_ts);
            }
        }
    }

    // Generate data types
    for (_id, dt) in &model.data_types {
        let ts = generate_datatype(dt, model);
        if let Some(pt) = pkg_tokens.get_mut(&dt.package_id) {
            pt.tokens.push(ts);
        }
        // Also generate FromGml
        if with_reader {
            let reader_ts = from_gml_gen::generate_from_gml_datatype(dt, model);
            if !reader_ts.is_empty() {
                if let Some(pt) = pkg_tokens.get_mut(&dt.package_id) {
                    pt.tokens.push(reader_ts);
                }
            }
        }
    }

    // Generate classes in topological order
    for class_id in &model.sorted_class_ids {
        if let Some(cls) = model.classes.get(class_id.as_str()) {
            let ts = if cls.is_abstract {
                generate_trait(cls, model)
            } else if cls.own_properties.is_empty()
                && cls.parent_ids.is_empty()
                && !cls.is_abstract
            {
                // CodeList class (no attrs, no parents, concrete)
                generate_codelist(cls)
            } else {
                generate_struct(cls, model)
            };

            if let Some(pt) = pkg_tokens.get_mut(&cls.package_id) {
                pt.tokens.push(ts);
            }

            // Also generate FromGml for non-abstract classes
            if with_reader && !cls.is_abstract {
                let reader_ts = if cls.own_properties.is_empty()
                    && cls.parent_ids.is_empty()
                {
                    from_gml_gen::generate_from_gml_codelist(&cls.name)
                } else {
                    from_gml_gen::generate_from_gml_class(cls, model)
                };
                if let Some(pt) = pkg_tokens.get_mut(&cls.package_id) {
                    pt.tokens.push(reader_ts);
                }
            }
        }
    }

    // Write files
    let mut module_names = Vec::new();

    for (_pkg_id, pt) in &pkg_tokens {
        if pt.tokens.is_empty() {
            continue;
        }

        let combined: TokenStream = pt.tokens.iter().cloned().collect();

        // Format with prettyplease
        let mut file_content = format_tokens(combined)?;

        // Prepend cross-module imports
        file_content = format!(
            "#![allow(unused_imports, unused_mut, unused_variables)]\nuse super::*;\n\n{file_content}"
        );

        let filename = format!("{}.rs", pt.module_name);
        module_names.push(pt.module_name.clone());

        if dry_run {
            if verbose {
                eprintln!("  [dry-run] Would write {filename} ({} bytes)", file_content.len());
            }
        } else {
            let file_path = output_dir.join(&filename);
            if verbose {
                eprintln!("  Writing {filename}...");
            }
            std::fs::write(&file_path, &file_content)?;
        }
    }

    // Generate dispatchers module if with_reader
    if with_reader {
        let dispatcher_tokens = from_gml_gen::generate_dispatchers(model);
        if !dispatcher_tokens.is_empty() {
            let mut file_content = format_tokens(dispatcher_tokens)?;
            file_content = format!(
                "#![allow(unused_imports, unused_mut, unused_variables)]\nuse super::*;\n\n{file_content}"
            );
            module_names.push("dispatchers".to_string());

            if dry_run {
                if verbose {
                    eprintln!("  [dry-run] Would write dispatchers.rs ({} bytes)", file_content.len());
                }
            } else {
                let file_path = output_dir.join("dispatchers.rs");
                if verbose {
                    eprintln!("  Writing dispatchers.rs...");
                }
                std::fs::write(&file_path, &file_content)?;
            }
        }
    }

    Ok(module_names)
}

fn format_tokens(tokens: TokenStream) -> Result<String, GenError> {
    let file = syn::parse2::<syn::File>(tokens)
        .map_err(|e| GenError::Codegen(format!("Failed to parse generated tokens: {e}")))?;
    Ok(prettyplease::unparse(&file))
}
