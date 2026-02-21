use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;

use crate::codegen::mapper::{prop_method_ident, prop_to_return_type, trait_ident, type_ident};
use crate::ir::model::UmlModel;
use crate::ir::types::UmlClass;
use crate::util::naming::{escape_keyword, to_snake_case};

/// Threshold: if an enum has more than this many variants, box them to control stack size.
const BOX_THRESHOLD: usize = 8;

/// Generate an enum dispatch type for an abstract class, plus trait impl and From impls.
///
/// The enum has one variant per concrete descendant. Variants are boxed when the
/// total variant count exceeds `BOX_THRESHOLD`.
pub fn generate_enum_dispatch(
    abs_cls: &UmlClass,
    descendants: &[&UmlClass],
    model: &UmlModel,
) -> TokenStream {
    let enum_name = type_ident(&abs_cls.name);
    let use_box = descendants.len() > BOX_THRESHOLD;

    // ── Enum definition ──
    let variants: Vec<TokenStream> = descendants
        .iter()
        .map(|cls| {
            let variant_name = type_ident(&cls.name);
            if use_box {
                quote! { #variant_name(Box<#variant_name>) }
            } else {
                quote! { #variant_name(#variant_name) }
            }
        })
        .collect();

    // Check if all descendants are cloneable
    let non_clone = model.non_cloneable_ids();
    let all_cloneable = descendants.iter().all(|cls| !non_clone.contains(&cls.xmi_id));
    let enum_def = if all_cloneable {
        quote! {
            #[derive(Debug, Clone)]
            pub enum #enum_name {
                #(#variants,)*
            }
        }
    } else {
        quote! {
            #[derive(Debug)]
            pub enum #enum_name {
                #(#variants,)*
            }
        }
    };

    // ── Default impl (returns first variant's default) ──
    let first_descendant = type_ident(&descendants[0].name);
    let default_impl = if use_box {
        quote! {
            impl Default for #enum_name {
                fn default() -> Self {
                    Self::#first_descendant(Box::default())
                }
            }
        }
    } else {
        quote! {
            impl Default for #enum_name {
                fn default() -> Self {
                    Self::#first_descendant(Default::default())
                }
            }
        }
    };

    // ── Trait impl on the enum ──
    let trait_impl = generate_trait_impl_on_enum(abs_cls, descendants, model);

    // ── From<ConcreteType> impls ──
    let from_impls: Vec<TokenStream> = descendants
        .iter()
        .map(|cls| {
            let variant_name = type_ident(&cls.name);
            if use_box {
                quote! {
                    impl From<#variant_name> for #enum_name {
                        fn from(v: #variant_name) -> Self {
                            Self::#variant_name(Box::new(v))
                        }
                    }
                }
            } else {
                quote! {
                    impl From<#variant_name> for #enum_name {
                        fn from(v: #variant_name) -> Self {
                            Self::#variant_name(v)
                        }
                    }
                }
            }
        })
        .collect();

    quote! {
        #enum_def
        #default_impl
        #trait_impl
        #(#from_impls)*
    }
}

/// Generate `impl Trait for Enum` that delegates each method via match to the inner struct.
fn generate_trait_impl_on_enum(
    abs_cls: &UmlClass,
    descendants: &[&UmlClass],
    model: &UmlModel,
) -> TokenStream {
    let enum_name = type_ident(&abs_cls.name);
    let own_trait_name = trait_ident(&abs_cls.name);

    // Collect all methods defined on this trait (own properties that are NOT
    // already in an ancestor trait, mirroring trait_gen.rs logic).
    let mut ancestor_methods: HashSet<String> = HashSet::new();
    let ancestors = model.ancestor_chain(&abs_cls.xmi_id);
    for ancestor in &ancestors {
        if ancestor.is_abstract {
            for prop in &ancestor.own_properties {
                ancestor_methods.insert(escape_keyword(&to_snake_case(&prop.name)));
            }
        }
    }

    // Filter to own properties not shadowed by ancestors, and not skipped
    let own_methods: Vec<TokenStream> = abs_cls
        .own_properties
        .iter()
        .filter(|prop| {
            let method_key = escape_keyword(&to_snake_case(&prop.name));
            !ancestor_methods.contains(&method_key) && !model.should_skip_prop(prop)
        })
        .map(|prop| {
            let method_name = prop_method_ident(&prop.name);
            let return_type = prop_to_return_type(prop, model);
            let match_arms: Vec<TokenStream> = descendants
                .iter()
                .map(|cls| {
                    let variant_name = type_ident(&cls.name);
                    quote! { Self::#variant_name(v) => v.#method_name() }
                })
                .collect();
            quote! {
                fn #method_name(&self) -> #return_type {
                    match self {
                        #(#match_arms,)*
                    }
                }
            }
        })
        .collect();

    // Also generate impls for all ancestor traits
    let mut all_trait_impls = Vec::new();

    // Ancestor trait impls (root first)
    for ancestor in ancestors.iter().rev() {
        if !ancestor.is_abstract {
            continue;
        }
        let anc_trait_name = trait_ident(&ancestor.name);

        // Collect methods declared on this ancestor trait
        let mut anc_ancestor_methods: HashSet<String> = HashSet::new();
        let anc_ancestors = model.ancestor_chain(&ancestor.xmi_id);
        for anc_anc in &anc_ancestors {
            if anc_anc.is_abstract {
                for prop in &anc_anc.own_properties {
                    anc_ancestor_methods.insert(escape_keyword(&to_snake_case(&prop.name)));
                }
            }
        }

        let methods: Vec<TokenStream> = ancestor
            .own_properties
            .iter()
            .filter(|prop| {
                let method_key = escape_keyword(&to_snake_case(&prop.name));
                !anc_ancestor_methods.contains(&method_key) && !model.should_skip_prop(prop)
            })
            .map(|prop| {
                let method_name = prop_method_ident(&prop.name);
                let return_type = prop_to_return_type(prop, model);
                let match_arms: Vec<TokenStream> = descendants
                    .iter()
                    .map(|cls| {
                        let variant_name = type_ident(&cls.name);
                        quote! { Self::#variant_name(v) => v.#method_name() }
                    })
                    .collect();
                quote! {
                    fn #method_name(&self) -> #return_type {
                        match self {
                            #(#match_arms,)*
                        }
                    }
                }
            })
            .collect();

        all_trait_impls.push(quote! {
            impl #anc_trait_name for #enum_name {
                #(#methods)*
            }
        });
    }

    // The trait impl for the class's own trait
    all_trait_impls.push(quote! {
        impl #own_trait_name for #enum_name {
            #(#own_methods)*
        }
    });

    quote! { #(#all_trait_impls)* }
}
