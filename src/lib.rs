#![warn(
    clippy::all,
    clippy::pedantic,
    future_incompatible,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    nonstandard_style,
    noop_method_call,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    trivial_casts,
    unreachable_pub,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused,
    variant_size_differences
)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::items_after_statements,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::unreadable_literal
)]

use std::collections::BTreeMap;

use eyre::Result;

use crate::stats::sample::Sample;

pub mod stats;

// Group of samples of the same type to compare together.
// TODO: Use ANOVA?
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SampleGroup {
    name: String,
    samples: BTreeMap<String, Sample>,
}

impl std::fmt::Display for SampleGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, v) in &self.samples {
            writeln!(f, "  {}: {}", k, v)?;
        }
        Ok(())
    }
}

impl SampleGroup {
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self { name: name.to_owned(), samples: BTreeMap::new() }
    }

    // Adds the sampled value to the Sample with name |id|.cd
    pub fn add(&mut self, id: &str, v: f64) {
        self.samples.entry(id.to_owned()).or_insert_with(Sample::new).add(v);
    }

    pub fn analyse(&self) -> Result<f64> {
        // TODO: assumes there are two things here
        let mut iter = self.samples.iter();
        let a = iter.next().unwrap().1;
        let b = iter.next().unwrap().1;
        a.ttest(b)
    }
}

// Time-series like object.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Series;

// Grapher performs analysis and draws graphs of samples and sample groups.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Grapher {
    groups: BTreeMap<String, SampleGroup>,
}

impl Grapher {
    #[must_use]
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
                println!("group {}, p {:.4}:\n{}", k, p, v);
            }
        }
    }
}
