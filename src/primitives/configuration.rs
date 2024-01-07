/// All the possible forms of a configuration
#[derive(Debug)]
pub enum Configuration {
    /// A boolean configuration, either `true` or `false`
    Boolean(bool),
    /// String configuration that can hold an arbitrary string
    String(String),
}
