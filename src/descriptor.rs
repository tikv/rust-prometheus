use crate::{Error, Result};

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
}
