#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Crate {
    /// Add USE_CLAP and CLAP script
    Clap,
    /// Add USE_SERDE and SERDE script
    Serde,
    /// Add USE_RATATUI and RATATUI script with crossterm
    Ratatui,
    /// Add USE_CROSSTERM and CROSSTERM script only
    Crossterm,
    /// Create only tokio::main
    Tokio,
    /// Try to add this crate if it exists
    Other(String),
}
impl Crate {
    pub fn new(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "clap" => Crate::Clap,
            "serde" => Crate::Serde,
            "ratatui" => Crate::Ratatui,
            "crossterm" => Crate::Crossterm,
            "tokio" => Crate::Tokio,
            _ => Crate::Other(s.to_string()),
        }
    }
}
