use std::collections::{HashMap, HashSet};

use indexmap::IndexMap;

use super::types::*;

/// The complete UML model after resolution.
#[derive(Debug, Clone)]
pub struct UmlModel {
    pub packages: IndexMap<String, UmlPackage>,
    pub classes: IndexMap<String, UmlClass>,
    pub enumerations: IndexMap<String, UmlEnum>,
    pub data_types: IndexMap<String, UmlDataType>,
    /// Topologically sorted class IDs (parents before children).
    pub sorted_class_ids: Vec<String>,

    // ── Pre-computed caches (populated by `compute_caches`) ──

    /// class_id → ordered ancestor IDs (direct parent first → root last).
    ancestor_cache: HashMap<String, Vec<String>>,
    /// abstract_class_id → concrete descendant IDs.
    descendant_cache: HashMap<String, Vec<String>>,
    /// Set of (class/datatype ID, property name) pairs that should be skipped.
    skip_props: HashSet<(String, String)>,
}

impl UmlModel {
    /// Build caches after the model is fully constructed.
    /// Must be called once before any codegen queries.
    pub fn compute_caches(&mut self) {
        self.build_ancestor_cache();
        self.build_descendant_cache();
        self.build_skip_prop_cache();
    }

    fn build_ancestor_cache(&mut self) {
        let class_ids: Vec<String> = self.classes.keys().cloned().collect();
        for id in &class_ids {
            if !self.ancestor_cache.contains_key(id) {
                let chain = self.compute_ancestor_chain(id);
                self.ancestor_cache.insert(id.clone(), chain);
            }
        }
    }

    fn compute_ancestor_chain(&self, id: &str) -> Vec<String> {
        let mut chain = Vec::new();
        let mut visited = HashSet::new();
        self.collect_ancestor_ids(id, &mut chain, &mut visited);
        chain
    }

    fn collect_ancestor_ids(
        &self,
        id: &str,
        chain: &mut Vec<String>,
        visited: &mut HashSet<String>,
    ) {
        if let Some(cls) = self.classes.get(id) {
            for parent_id in &cls.parent_ids {
                if visited.insert(parent_id.clone()) && self.classes.contains_key(parent_id.as_str())
                {
                    chain.push(parent_id.clone());
                    self.collect_ancestor_ids(parent_id, chain, visited);
                }
            }
        }
    }

    fn build_descendant_cache(&mut self) {
        // For each abstract class, find all concrete classes that have it as an ancestor
        let abstract_ids: Vec<String> = self
            .classes
            .values()
            .filter(|c| c.is_abstract)
            .map(|c| c.xmi_id.clone())
            .collect();

        for abs_id in &abstract_ids {
            self.descendant_cache
                .entry(abs_id.clone())
                .or_default();
        }

        for cls in self.classes.values() {
            if cls.is_abstract {
                continue;
            }
            let ancestors = self
                .ancestor_cache
                .get(&cls.xmi_id)
                .cloned()
                .unwrap_or_default();
            for anc_id in &ancestors {
                if let Some(desc) = self.descendant_cache.get_mut(anc_id) {
                    desc.push(cls.xmi_id.clone());
                }
            }
        }
    }

    fn build_skip_prop_cache(&mut self) {
        // Pre-compute which type IDs should cause a property to be skipped
        let mut skip_type_ids: HashSet<String> = HashSet::new();

        // ADEOf* data types
        for (id, dt) in &self.data_types {
            if dt.name.starts_with("ADEOf") {
                skip_type_ids.insert(id.clone());
            }
        }

        // Abstract data types with no properties
        for (id, dt) in &self.data_types {
            if dt.is_abstract && dt.properties.is_empty() {
                skip_type_ids.insert(id.clone());
            }
        }

        // Abstract classes with 0 concrete descendants
        for (id, cls) in &self.classes {
            if cls.is_abstract {
                let descendants = self.descendant_cache.get(id).map_or(0, |d| d.len());
                if descendants == 0 {
                    skip_type_ids.insert(id.clone());
                }
            }
        }

        // Now scan all properties across all classes and data types
        for cls in self.classes.values() {
            for prop in &cls.own_properties {
                if let UmlTypeRef::Known(type_id) = &prop.type_ref
                    && skip_type_ids.contains(type_id)
                {
                    self.skip_props
                        .insert((cls.xmi_id.clone(), prop.name.clone()));
                }
            }
        }

        for dt in self.data_types.values() {
            for prop in &dt.properties {
                if let UmlTypeRef::Known(type_id) = &prop.type_ref
                    && skip_type_ids.contains(type_id)
                {
                    self.skip_props
                        .insert((dt.xmi_id.clone(), prop.name.clone()));
                }
            }
        }
    }

    /// Full ancestor chain for a class: direct parent first → root last.
    pub fn ancestor_chain(&self, id: &str) -> Vec<&UmlClass> {
        match self.ancestor_cache.get(id) {
            Some(ids) => ids
                .iter()
                .filter_map(|aid| self.classes.get(aid.as_str()))
                .collect(),
            None => {
                // Fallback for uncached (shouldn't happen after compute_caches)
                let computed = self.compute_ancestor_chain(id);
                computed
                    .iter()
                    .filter_map(|aid| self.classes.get(aid.as_str()))
                    .collect()
            }
        }
    }

    /// All properties for a class: ancestor root first, own properties last.
    /// No duplicates — properties from the root ancestor come first,
    /// then intermediate ancestors, then own.
    pub fn all_properties(&self, id: &str) -> Vec<&UmlProperty> {
        let ancestors = self.ancestor_chain(id);
        let mut props = Vec::new();

        // Ancestors in reverse (root first)
        for ancestor in ancestors.iter().rev() {
            for prop in &ancestor.own_properties {
                props.push(prop);
            }
        }

        // Own properties
        if let Some(cls) = self.classes.get(id) {
            for prop in &cls.own_properties {
                props.push(prop);
            }
        }

        props
    }

    /// Get the package name for a given package ID.
    pub fn package_name(&self, package_id: &str) -> Option<&str> {
        self.packages.get(package_id).map(|p| p.name.as_str())
    }

    /// Determine if a class (by xmi_id) is abstract.
    pub fn is_abstract(&self, id: &str) -> bool {
        self.classes.get(id).is_some_and(|c| c.is_abstract)
    }

    /// All non-abstract classes that have the given abstract class as an ancestor.
    pub fn concrete_descendants(&self, abstract_id: &str) -> Vec<&UmlClass> {
        match self.descendant_cache.get(abstract_id) {
            Some(ids) => ids
                .iter()
                .filter_map(|id| self.classes.get(id.as_str()))
                .collect(),
            None => Vec::new(),
        }
    }

    /// Returns true if the data type is an ADEOf* extension type.
    pub fn is_ade_type(&self, id: &str) -> bool {
        self.data_types
            .get(id)
            .is_some_and(|dt| dt.name.starts_with("ADEOf"))
    }

    /// Returns true if a property should be skipped during code generation.
    /// Uses the pre-computed skip set when an owner_id is provided.
    pub fn should_skip_prop(&self, prop: &UmlProperty) -> bool {
        match &prop.type_ref {
            UmlTypeRef::Known(id) => {
                // ADEOf* data types → skip
                if self.is_ade_type(id) {
                    return true;
                }
                // Abstract data types with no properties and no concrete impls → skip
                if let Some(dt) = self.data_types.get(id.as_str())
                    && dt.is_abstract
                    && dt.properties.is_empty()
                {
                    return true;
                }
                // Abstract classes with 0 concrete descendants → skip
                if let Some(cls) = self.classes.get(id.as_str())
                    && cls.is_abstract
                {
                    let desc_count = self.descendant_cache.get(id).map_or(0, |d| d.len());
                    if desc_count == 0 {
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }

    /// Fast skip check using the pre-computed set.
    /// `owner_id` is the xmi_id of the class or data type that owns the property.
    pub fn should_skip_prop_fast(&self, owner_id: &str, prop: &UmlProperty) -> bool {
        self.skip_props.contains(&(owner_id.to_string(), prop.name.clone()))
    }

    /// Compute the set of class/data-type IDs whose generated structs/enums cannot derive Clone.
    /// A type is non-cloneable if it (transitively) contains a `Box<dyn Any>` field
    /// (from GmObject/AnyFeature external types).
    pub fn non_cloneable_ids(&self) -> HashSet<String> {
        let mut non_clone: HashSet<String> = HashSet::new();

        // Helper: check if a single property makes its owner non-cloneable
        let prop_is_non_clone = |prop: &UmlProperty, nc: &HashSet<String>| -> bool {
            match &prop.type_ref {
                UmlTypeRef::Known(id) => nc.contains(id),
                _ => false,
            }
        };

        // Data types first (no inheritance, simple)
        for (id, dt) in &self.data_types {
            if dt.properties.iter().any(|p| prop_is_non_clone(p, &non_clone)) {
                non_clone.insert(id.clone());
            }
        }

        // Classes in topological order (parents before children)
        for class_id in &self.sorted_class_ids {
            if let Some(cls) = self.classes.get(class_id.as_str()) {
                if cls.is_abstract {
                    // Abstract classes become enums — non-cloneable if any descendant is
                    let descendants = self.concrete_descendants(class_id);
                    if !descendants.is_empty()
                        && descendants.iter().any(|d| non_clone.contains(&d.xmi_id))
                    {
                        non_clone.insert(class_id.clone());
                    }
                    continue;
                }
                let all_props = self.all_properties(class_id);
                if all_props.iter().any(|p| {
                    !self.should_skip_prop(p) && prop_is_non_clone(p, &non_clone)
                }) {
                    non_clone.insert(class_id.clone());
                }
            }
        }

        non_clone
    }
}

/// Create an empty `UmlModel` — caches are empty until `compute_caches()` is called.
pub fn new_uml_model(
    packages: IndexMap<String, UmlPackage>,
    classes: IndexMap<String, UmlClass>,
    enumerations: IndexMap<String, UmlEnum>,
    data_types: IndexMap<String, UmlDataType>,
    sorted_class_ids: Vec<String>,
) -> UmlModel {
    UmlModel {
        packages,
        classes,
        enumerations,
        data_types,
        sorted_class_ids,
        ancestor_cache: HashMap::new(),
        descendant_cache: HashMap::new(),
        skip_props: HashSet::new(),
    }
}
