use std::collections::VecDeque;

use indexmap::{IndexMap, IndexSet};

use crate::ir::types::UmlClass;

/// Topological sort of class graph using Kahn's algorithm.
/// Returns class IDs in order: parents before children.
pub fn topological_sort(classes: &IndexMap<String, UmlClass>) -> Vec<String> {
    // Build adjacency: parent → children
    let mut in_degree: IndexMap<&str, usize> = IndexMap::new();
    let mut children: IndexMap<&str, Vec<&str>> = IndexMap::new();

    for (id, _) in classes {
        in_degree.entry(id.as_str()).or_insert(0);
        children.entry(id.as_str()).or_default();
    }

    for (id, cls) in classes {
        for parent_id in &cls.parent_ids {
            if classes.contains_key(parent_id.as_str()) {
                *in_degree.entry(id.as_str()).or_insert(0) += 1;
                children.entry(parent_id.as_str()).or_default().push(id.as_str());
            }
        }
    }

    // Kahn's algorithm
    let mut queue: VecDeque<&str> = VecDeque::new();
    for (id, &deg) in &in_degree {
        if deg == 0 {
            queue.push_back(id);
        }
    }

    let mut sorted = Vec::with_capacity(classes.len());
    let mut visited = IndexSet::new();

    while let Some(id) = queue.pop_front() {
        if !visited.insert(id) {
            continue;
        }
        sorted.push(id.to_string());

        if let Some(kids) = children.get(id) {
            for &child_id in kids {
                if let Some(deg) = in_degree.get_mut(child_id) {
                    *deg = deg.saturating_sub(1);
                    if *deg == 0 {
                        queue.push_back(child_id);
                    }
                }
            }
        }
    }

    // Add any remaining classes (cycles or disconnected) at the end
    for id in classes.keys() {
        if !visited.contains(id.as_str()) {
            sorted.push(id.clone());
        }
    }

    sorted
}
