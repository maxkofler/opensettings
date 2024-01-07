use std::str::FromStr;

/// An identifier that is constructed from multiple fields
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    /// The parts of the identifier
    pub identifier: Vec<String>,
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Self {
            identifier: value.split('.').map(str::to_owned).collect(),
        }
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self {
            identifier: value.split('.').map(str::to_owned).collect(),
        }
    }
}

impl FromStr for Identifier {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            identifier: s.split('.').map(str::to_owned).collect(),
        })
    }
}

impl ToString for Identifier {
    fn to_string(&self) -> String {
        self.identifier.join(".")
    }
}
