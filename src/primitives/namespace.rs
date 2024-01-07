use std::collections::HashMap;

use crate::{Configuration, Identifier};

/// A configuration namespace to store multiple configurations in
#[derive(Debug)]
pub struct Namespace {
    pub configurations: HashMap<Identifier, Configuration>,
}

impl Namespace {
    /// Creates a new empty namespace with no configurations
    pub fn new() -> Self {
        Self {
            configurations: HashMap::new(),
        }
    }

    /// Sets configuration, returning the old one, if present
    /// # Arguments
    /// * `identifier` - The identifier for the configuration
    /// * `configuration` - The configuration to set
    pub fn set(
        &mut self,
        identifier: Identifier,
        configuration: Configuration,
    ) -> Option<Configuration> {
        self.configurations.insert(identifier, configuration)
    }
}
