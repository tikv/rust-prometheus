use crate::{Error, Result};

/// The metric descriptor. It is essentially the immutable meta-data of a metric.
pub struct Descriptor {
    name: String,
    help: String,
    ordered_const_labels: Vec<(String, String)>,
    ordered_dynamic_label_names: Vec<String>,
}

impl Descriptor {
    pub fn new(
        name: String,
        help: String,
        mut const_labels: Vec<(String, String)>,
        mut dynamic_label_names: Vec<String>,
    ) -> Result<Self> {
        const_labels.sort();
        dynamic_label_names.sort();
        Ok(Self {
            name,
            help,
            ordered_const_labels: const_labels,
            ordered_dynamic_label_names: dynamic_label_names,
        })
    }

    /// Gets the name of the metric.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the help of the metric.
    pub fn help(&self) -> &str {
        &self.help
    }
}
