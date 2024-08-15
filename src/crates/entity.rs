use crate::helpers::*;
use anyhow::anyhow;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrateName {
    /// Add USE_CLAP and CLAP script only with parser feature
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
impl From<&str> for CrateName {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "clap" => CrateName::Clap,
            "serde" => CrateName::Serde,
            "ratatui" => CrateName::Ratatui,
            "crossterm" => CrateName::Crossterm,
            "tokio" => CrateName::Tokio,
            _ => CrateName::Other(s.to_string()),
        }
    }
}
impl From<String> for CrateName {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "clap" => CrateName::Clap,
            "serde" => CrateName::Serde,
            "ratatui" => CrateName::Ratatui,
            "crossterm" => CrateName::Crossterm,
            "tokio" => CrateName::Tokio,
            _ => CrateName::Other(s.to_string()),
        }
    }
}
#[derive(Debug)]
pub struct Crate {
    pub name: CrateName,
    pub features: Option<Vec<String>>,
    pub link: String,
    pub use_snippet: Option<String>,
    pub core_snippet: Option<String>,
}
impl From<&str> for Crate {
    fn from(s: &str) -> Self {
        let name = CrateName::from(s);
        let link = get_crate_link(s);
        let (use_snippet, core_snippet) = match name {
            CrateName::Clap => (snippets::USE_CLAP, snippets::CLAP),
            CrateName::Serde => (snippets::USE_SERDE, snippets::SERDE),
            CrateName::Ratatui => (snippets::USE_RATATUI, snippets::RATATUI),
            CrateName::Crossterm => (snippets::USE_CROSSTERM, snippets::CROSSTERM),
            CrateName::Tokio => ("", snippets::TOKIO),
            CrateName::Other(_) => ("", ""),
        };
        Self {
            name,
            features: None,
            link,
            use_snippet: if use_snippet.is_empty() {
                None
            } else {
                Some(use_snippet.to_string())
            },
            core_snippet: if core_snippet.is_empty() {
                None
            } else {
                Some(core_snippet.to_string())
            },
        }
    }
}
impl From<CrateOption> for Crate {
    fn from(co: CrateOption) -> Self {
        let (use_snippet, core_snippet) = match co.path_to_snippets {
            Some(path_to_snippet) => create_snippets(&path_to_snippet),
            None => (None, None),
        };
        Self {
            name: CrateName::from(co.name.as_str()),
            features: co.features,
            link: if let Some(link) = co.link {
                link
            } else {
                get_crate_link(&co.name)
            },
            use_snippet,
            core_snippet,
        }
    }
}
impl Crate {
    pub fn add_crate(&self, path: &str) -> anyhow::Result<()> {
        let res = match &self.name {
            CrateName::Clap => {
                let mut com = std::process::Command::new("cargo")
                    .args(["add", "clap", "-F", "derive"])
                    .current_dir(path)
                    .spawn()?;

                com.wait()?.success()
            }
            CrateName::Serde => {
                let mut com = std::process::Command::new("cargo")
                    .args(["add", "serde", "-F", "derive"])
                    .current_dir(path)
                    .spawn()?;
                let s1 = com.wait()?.success();
                let mut com = std::process::Command::new("cargo")
                    .args(["add", "serde_json"])
                    .current_dir(path)
                    .spawn()?;
                com.wait()?.success() && s1
            }
            CrateName::Tokio => {
                let mut com = std::process::Command::new("cargo")
                    .args(["add", "tokio", "-F", "full"])
                    .current_dir(path)
                    .spawn()?;

                com.wait()?.success()
            }
            CrateName::Ratatui => {
                let mut com = std::process::Command::new("cargo")
                    .args(["add", "ratatui"])
                    .current_dir(path)
                    .spawn()?;
                com.wait()?.success()
            }
            CrateName::Crossterm => {
                let mut com = std::process::Command::new("cargo")
                    .args(["add", "crossterm"])
                    .current_dir(path)
                    .spawn()?;
                com.wait()?.success()
            }
            CrateName::Other(s) => {
                let mut com = std::process::Command::new("cargo")
                    .args(["add", s.as_str()])
                    .current_dir(path)
                    .spawn()?;
                com.wait()?.success()
            }
        };
        if res {
            Ok(())
        } else {
            Err(anyhow!("add crate {:?} error", self.name))
        }
    }
    pub fn add_use(&self, path: &str) -> anyhow::Result<()> {
        if let Some(use_snippet) = &self.use_snippet {
            let mut file = std::fs::File::open(path)?;
            writeln!(file, "{}", use_snippet)?
        }
        Ok(())
    }
    pub fn add_core(&self, path: &str) -> anyhow::Result<()> {
        if let Some(core_snippet) = &self.core_snippet {
            let mut file = std::fs::File::open(path)?;
            writeln!(file, "{}", core_snippet)?
        }
        Ok(())
    }
}
