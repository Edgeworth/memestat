use std::collections::BTreeMap;

use crate::stats::sample_group::SampleGroup;

/// Grapher performs analysis and draws graphs of samples and sample groups.
#[must_use]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Analyze {
    groups: BTreeMap<String, SampleGroup>,
}

impl Analyze {
    pub fn new() -> Self {
        Self { groups: BTreeMap::new() }
    }

    // Adds the sampled value to the Sample with name |id| in group |group|.
    pub fn add(&mut self, group: &str, id: &str, v: f64) {
        self.groups.entry(group.to_owned()).or_insert_with(|| SampleGroup::new(group)).add(id, v);
    }

    pub fn analyse(&self) {
        for (k, v) in &self.groups {
            if let Ok(p) = v.analyse() {
                println!("group {k}, p {p:.4}:\n{v}");
            }
        }
    }
}
