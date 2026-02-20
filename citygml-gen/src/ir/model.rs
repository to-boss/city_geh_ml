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
}

impl UmlModel {
    /// Full ancestor chain for a class: direct parent first → root last.
    pub fn ancestor_chain(&self, id: &str) -> Vec<&UmlClass> {
        let mut chain = Vec::new();
        let mut visited = std::collections::HashSet::new();
        self.collect_ancestors(id, &mut chain, &mut visited);
        chain
    }

    fn collect_ancestors<'a>(
        &'a self,
        id: &str,
        chain: &mut Vec<&'a UmlClass>,
        visited: &mut std::collections::HashSet<String>,
    ) {
        if let Some(cls) = self.classes.get(id) {
            for parent_id in &cls.parent_ids {
                if visited.insert(parent_id.clone()) {
                    if let Some(parent) = self.classes.get(parent_id.as_str()) {
                        chain.push(parent);
                        self.collect_ancestors(parent_id, chain, visited);
                    }
                }
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
        let mut result = Vec::new();
        for cls in self.classes.values() {
            if cls.is_abstract {
                continue;
            }
            let ancestors = self.ancestor_chain(&cls.xmi_id);
            if ancestors.iter().any(|a| a.xmi_id == abstract_id) {
                result.push(cls);
            }
        }
        result
    }

    /// Returns true if the data type is an ADEOf* extension type.
    pub fn is_ade_type(&self, id: &str) -> bool {
        self.data_types
            .get(id)
            .is_some_and(|dt| dt.name.starts_with("ADEOf"))
    }

    /// Returns true if a property should be skipped during code generation:
    /// - ADEOf* data type references
    /// - Abstract data types with 0 properties (dead abstract types like AbstractGenericAttribute)
    /// - Abstract classes with 0 concrete descendants
    pub fn should_skip_prop(&self, prop: &UmlProperty) -> bool {
        match &prop.type_ref {
            UmlTypeRef::Known(id) => {
                // ADEOf* data types → skip
                if self.is_ade_type(id) {
                    return true;
                }
                // Abstract data types with no properties and no concrete impls → skip
                if let Some(dt) = self.data_types.get(id.as_str()) {
                    if dt.is_abstract && dt.properties.is_empty() {
                        return true;
                    }
                }
                // Abstract classes with 0 concrete descendants → skip
                if let Some(cls) = self.classes.get(id.as_str()) {
                    if cls.is_abstract && self.concrete_descendants(id).is_empty() {
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }

    /// Compute the set of class/data-type IDs whose generated structs/enums cannot derive Clone.
    /// A type is non-cloneable if it (transitively) contains a `Box<dyn Any>` field
    /// (from GmObject/AnyFeature external types).
    pub fn non_cloneable_ids(&self) -> std::collections::HashSet<String> {
        let mut non_clone: std::collections::HashSet<String> = std::collections::HashSet::new();

        // Helper: check if a single property makes its owner non-cloneable
        // (transitively contains a non-cloneable known type)
        let prop_is_non_clone = |prop: &UmlProperty, nc: &std::collections::HashSet<String>| -> bool {
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
