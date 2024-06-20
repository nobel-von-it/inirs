use clap::Parser;

use crate::{crates::Crate, snippets};
use std::io::Write;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, default_value = "/home/nerd/Dev/Rusty/")]
    dir: String,
    #[clap(short, long)]
    name: String,
    crates: Vec<String>,
}
impl Args {
    pub fn create_by_name(&self) {
        let _ = std::fs::create_dir_all(self.dir.clone());
        let mut com = std::process::Command::new("cargo")
            .args(["new", "--bin", &self.name])
            .current_dir(&self.dir)
            .spawn()
            .unwrap();
        if com.wait().unwrap().success() {
            println!("{} created", self.name);
        }
    }
    pub fn add_all_crates(&self) {
        let path = format!("{}{}", &self.dir, &self.name);
        let path_to_main = format!("{}/src/main.rs", &path);
        let mut file = std::fs::File::create(path_to_main).unwrap();
        let crates = self
            .crates
            .iter()
            .map(|c| c.to_lowercase())
            .collect::<Vec<String>>();

        for crate_ in crates.iter() {
            let crate_ = Crate::new(crate_);
            match crate_ {
                Crate::Clap => {
                    let mut com = std::process::Command::new("cargo")
                        .args(["add", "clap", "--features", "derive"])
                        .current_dir(&path)
                        .spawn()
                        .unwrap();
                    if com.wait().unwrap().success() {
                        println!("clap added");
                    }
                    let _ = file.write_all(snippets::USE_CLAP.as_bytes());
                }
                Crate::Serde => {
                    let mut com = std::process::Command::new("cargo")
                        .args(["add", "serde", "--features", "derive"])
                        .current_dir(&path)
                        .spawn()
                        .unwrap();
                    if com.wait().unwrap().success() {
                        println!("serde added");
                    }
                    let mut com = std::process::Command::new("cargo")
                        .args(["add", "serde_json"])
                        .current_dir(&path)
                        .spawn()
                        .unwrap();
                    if com.wait().unwrap().success() {
                        println!("serde_json added");
                    }
                    let _ = file.write_all(snippets::USE_SERDE.as_bytes());
                }
                Crate::Tokio => {
                    let mut com = std::process::Command::new("cargo")
                        .args(["add", "tokio", "--features", "full"])
                        .current_dir(&path)
                        .spawn()
                        .unwrap();
                    if com.wait().unwrap().success() {
                        println!("tokio added");
                    }
                }
                Crate::Ratatui => {
                    let mut com = std::process::Command::new("cargo")
                        .args(["add", "ratatui"])
                        .current_dir(&path)
                        .spawn()
                        .unwrap();
                    if com.wait().unwrap().success() {
                        println!("ratatui added");
                    }
                    let _ = file.write_all(snippets::USE_RATATUI.as_bytes());
                }
                Crate::Crossterm => {
                    let mut com = std::process::Command::new("cargo")
                        .args(["add", "crossterm"])
                        .current_dir(&path)
                        .spawn()
                        .unwrap();
                    if com.wait().unwrap().success() {
                        println!("crossterm added");
                    }
                    if !crates.contains(&"ratatui".to_string()) {
                        let _ = file.write_all(snippets::USE_CROSSTERM.as_bytes());
                    }
                }
                Crate::Other(s) => {
                    let mut com = std::process::Command::new("cargo")
                        .args(["add", s.as_str()])
                        .current_dir(&path)
                        .spawn()
                        .unwrap();
                    if com.wait().unwrap().success() {
                        println!("{} added", s);
                    }
                }
            }
        }
        for crate_ in crates.iter() {
            let crate_ = Crate::new(crate_);
            match crate_ {
                Crate::Clap => {
                    let _ = file.write_all(snippets::CLAP.as_bytes());
                }
                Crate::Serde => {
                    let _ = file.write_all(snippets::SERDE.as_bytes());
                }
                Crate::Ratatui => {
                    let _ = file.write_all(snippets::RATATUI.as_bytes());
                }
                Crate::Crossterm => {
                    if !crates.contains(&"ratatui".to_string()) {
                        let _ = file.write_all(snippets::CROSSTERM.as_bytes());
                    }
                }
                Crate::Tokio => {
                    if !crates.contains(&"crossterm".to_string())
                        && !crates.contains(&"ratatui".to_string())
                    {
                        let _ = file.write_all(snippets::TOKIO.as_bytes());
                    }
                }
                _ => {}
            }
        }
        // if ratatui, crossterm and tokio are not added then add default main script
        if !crates.contains(&"ratatui".to_string())
            && !crates.contains(&"crossterm".to_string())
            && !crates.contains(&"tokio".to_string())
        {
            let _ = file.write_all(snippets::DEFAULT.as_bytes());
        }

        println!("all third-party crates added");
        let mut com = std::process::Command::new("cargo")
            .args(["fmt"])
            .current_dir(&path)
            .spawn()
            .unwrap();
        if com.wait().unwrap().success() {
            println!("cargo fmt");
        }
    }
    pub fn build(&self) {
        let path = format!("{}{}", &self.dir, &self.name);
        let mut com = std::process::Command::new("cargo")
            .args(["build"])
            .current_dir(&path)
            .spawn()
            .unwrap();
        if com.wait().unwrap().success() {
            println!("{} built", self.name);
        }
    }
}
