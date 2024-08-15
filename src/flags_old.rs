use anyhow::anyhow;
use clap::{Parser, Subcommand};

use crate::{
    crates::Crate,
    helpers,
    opt::{CrateOption, CrateOptions},
};

#[derive(Subcommand, Debug)]
pub enum Actions {
    Add {
        name: String,
        #[clap(short, long, default_value = None)]
        link: Option<String>,
        #[clap(short, long, default_value = None)]
        path_to_snippets: Option<String>,
        #[clap(default_value = None)]
        features: Option<Vec<String>>,
    },
    CreateSnippet {
        path_to_snippets: String,
    },
    Do {
        #[clap(short, long, default_value = "./")]
        dir_path: String,
        #[clap(short, long, required = true)]
        name: String,
        #[clap(short, long, default_value = "false")]
        is_lib: bool,
        #[clap(short, long, default_value = "false")]
        add_link: bool,
        #[clap(short, long, default_value = "false")]
        build: bool,
        #[clap(default_value = None)]
        crates: Option<Vec<String>>,
    },
}
