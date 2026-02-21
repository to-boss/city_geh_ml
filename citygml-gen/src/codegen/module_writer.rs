use std::path::Path;

use indexmap::IndexMap;
use proc_macro2::TokenStream;

use crate::codegen::datatype_gen::generate_datatype;
use crate::codegen::enum_gen::generate_enum;
use crate::codegen::from_gml_gen;
use crate::codegen::enum_gen_dispatch::generate_enum_dispatch;
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
            if !reader_ts.is_empty()
                && let Some(pt) = pkg_tokens.get_mut(&dt.package_id)
            {
                pt.tokens.push(reader_ts);
            }
        }
    }

    // Generate classes in topological order
    for class_id in &model.sorted_class_ids {
        if let Some(cls) = model.classes.get(class_id.as_str()) {
            if cls.is_abstract {
                let ts = generate_trait(cls, model);
                if let Some(pt) = pkg_tokens.get_mut(&cls.package_id) {
                    pt.tokens.push(ts);
                }
                // Generate enum dispatch if there are concrete descendants
                let descendants = model.concrete_descendants(&cls.xmi_id);
                if !descendants.is_empty() {
                    let enum_ts = generate_enum_dispatch(cls, &descendants, model);
                    if let Some(pt) = pkg_tokens.get_mut(&cls.package_id) {
                        pt.tokens.push(enum_ts);
                    }
                }
            } else if cls.own_properties.is_empty()
                && cls.parent_ids.is_empty()
            {
                // CodeList class (no attrs, no parents, concrete)
                let ts = generate_codelist(cls);
                if let Some(pt) = pkg_tokens.get_mut(&cls.package_id) {
                    pt.tokens.push(ts);
                }
            } else {
                let ts = generate_struct(cls, model);
                if let Some(pt) = pkg_tokens.get_mut(&cls.package_id) {
                    pt.tokens.push(ts);
                }
            };

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

    // Collect files to format: (module_name, token_string)
    // Convert TokenStream → String on the main thread (cheap),
    // then parse + format from String in parallel threads (expensive).
    // This is needed because proc_macro2::TokenStream is !Send.
    let mut files_to_format: Vec<(String, String)> = Vec::new();

    for (_pkg_id, pt) in &pkg_tokens {
        if pt.tokens.is_empty() {
            continue;
        }
        let combined: TokenStream = pt.tokens.iter().cloned().collect();
        files_to_format.push((pt.module_name.clone(), combined.to_string()));
    }

    // Generate dispatchers module if with_reader
    if with_reader {
        let dispatcher_tokens = from_gml_gen::generate_dispatchers(model);
        if !dispatcher_tokens.is_empty() {
            files_to_format.push(("dispatchers".to_string(), dispatcher_tokens.to_string()));
        }
    }

    // Format + write files in parallel using scoped threads
    let results: Vec<Result<String, GenError>> = if dry_run {
        std::thread::scope(|s| {
            let handles: Vec<_> = files_to_format
                .into_iter()
                .map(|(module_name, token_str)| {
                    s.spawn(move || {
                        let file_content = format_str_with_header(&token_str)?;
                        if verbose {
                            eprintln!(
                                "  [dry-run] Would write {}.rs ({} bytes)",
                                module_name,
                                file_content.len()
                            );
                        }
                        Ok(module_name)
                    })
                })
                .collect();
            handles.into_iter().map(|h| h.join().unwrap()).collect()
        })
    } else {
        let output_dir = output_dir.to_path_buf();
        std::thread::scope(|s| {
            let handles: Vec<_> = files_to_format
                .into_iter()
                .map(|(module_name, token_str)| {
                    let output_dir = &output_dir;
                    s.spawn(move || {
                        let file_content = format_str_with_header(&token_str)?;
                        let filename = format!("{module_name}.rs");
                        let file_path = output_dir.join(&filename);
                        if verbose {
                            eprintln!("  Writing {filename}...");
                        }
                        std::fs::write(&file_path, &file_content)?;
                        Ok(module_name)
                    })
                })
                .collect();
            handles.into_iter().map(|h| h.join().unwrap()).collect()
        })
    };

    // Collect results, propagating any errors
    let mut module_names = Vec::new();
    for result in results {
        module_names.push(result?);
    }

    Ok(module_names)
}

fn format_str_with_header(token_str: &str) -> Result<String, GenError> {
    let file_content = format_str(token_str)?;
    Ok(format!(
        "#![allow(unused_imports, unused_mut, unused_variables)]\nuse super::*;\n\n{file_content}"
    ))
}

fn format_str(token_str: &str) -> Result<String, GenError> {
    let file = syn::parse_str::<syn::File>(token_str)
        .map_err(|e| GenError::Codegen(format!("Failed to parse generated tokens: {e}")))?;
    let formatted = prettyplease::unparse(&file);
    Ok(add_blank_lines_between_items(&formatted))
}

/// Insert a blank line after every top-level closing brace (`}` at column 0)
/// so that structs, impls, enums, and traits are visually separated.
fn add_blank_lines_between_items(source: &str) -> String {
    let mut result = String::with_capacity(source.len() + source.len() / 20);
    let mut prev_was_closing_brace = false;
    for line in source.lines() {
        if prev_was_closing_brace && !line.is_empty() {
            result.push('\n');
        }
        result.push_str(line);
        result.push('\n');
        prev_was_closing_brace = line == "}";
    }
    result
}
