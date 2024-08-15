use std::io::Write;

use anyhow::anyhow;

use crate::{
    helpers::{create_snippets, get_crate_link},
    opt::CrateOption,
    snippets,
};

pub struct Features(Vec<String>);
// impl From<CrateName> for Features {
//     fn from(cn: CrateName) -> Self {
//     let mut features = vec![];
//         match cn {
//             CrateName::Clap =>
//         }
//     }
// }
