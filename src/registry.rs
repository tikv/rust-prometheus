use crate::{Error, Result};

pub struct Registry;

impl Registry {
    pub const fn new() -> Self {
        Self
    }

    pub fn register(&mut self, collector: &crate::Collector) -> Result<()> {
        let _ = collector.box_clone();
        Ok(())
    }
}
