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
}
