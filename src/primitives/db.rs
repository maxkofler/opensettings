use std::collections::HashMap;

use crate::{Identifier, Namespace};

/// A database of configurations
#[derive(Debug)]
pub struct ConfigurationDB {
    /// The namespaces managed in this database
    pub namespaces: HashMap<Identifier, Namespace>,
}

impl ConfigurationDB {
    /// Creates a new empty configuration database
    pub fn new() -> Self {
        Self {
            namespaces: HashMap::new(),
        }
    }

    /// Adds a new namespace to the database using the supplied identifier
    /// # Arguments
    /// * `identifier` - The identifier to use for the namespace
    /// * `namespace` - The namespace to add to the database
    /// # Returns
    /// The old namespace associated with the identifier if a duplicate is found
    pub fn add_namespace(&mut self, identifier: Identifier, namespace: Namespace) -> Option<Namespace> {
        self.namespaces.insert(identifier, namespace)
    }
}