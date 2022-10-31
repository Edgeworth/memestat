use std::collections::BTreeMap;

use eyre::Result;

use crate::stats::sample::Sample;

/// Group of samples of the same type to compare together.
// TODO(1): Use ANOVA?
#[must_use]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SampleGroup {
    name: String,
    samples: BTreeMap<String, Sample>,
}

impl std::fmt::Display for SampleGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, v) in &self.samples {
            writeln!(f, "  {k}: {v}")?;
        }
        Ok(())
    }
}

impl SampleGroup {
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
