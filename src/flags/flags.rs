use clap::{Parser, Subcommand};

use crate::helpers;

use super::actions::Actions;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long, default_value = "/home/nerd/.config/inirs/")]
    pub config_path: String,
    #[clap(long, default_value = "config.json")]
    pub config_name: String,
    #[clap(subcommand)]
    action: Option<Actions>,
}
impl Args {
    pub fn control(&self) -> anyhow::Result<()> {
        helpers::check_and_create_dir(&self.config_path)?;
        helpers::check_and_create_file(&format!("{}{}", &self.config_path, &self.config_name))?;

        if let Some(act) = &self.action {
            let mut crate_options =
                CrateOptions::load_from_file(&self.config_path, &self.config_name);
            match act {
                Actions::Do {
                    dir_path,
                    name,
                    is_lib,
                    add_link,
                    build,
                    crates,
                } => {}
                Actions::Add {
                    name,
                    link,
                    path_to_snippets,
                    features,
                } => match crate_options.get_crate(name) {
                    Some(co) => {
                        println!("crate with name {} exists", &co.name);
                        std::process::exit(0);
                    }
                    None => crate_options.add_crate(CrateOption {
                        name: name.to_string(),
                        features: features.clone(),
                        link: link.clone(),
                        path_to_snippets: path_to_snippets.clone(),
                    }),
                },
                Actions::CreateSnippet {
                    path_to_snippets,
                    path_to_output,
                } => {
                    helpers::check_and_create_dir(&path_to_snippets)?;
                    helpers::check_and_create_dir(&path_to_output)?;

                    match helpers::create_snippets(&path_to_snippets)
                }
            }
            crate_options.save_to_file
        }
        Ok(())
    }

    pub fn add_action(&self) {
        if let Some(Actions::Add {
            name,
            link,
            path_to_snippets,
            features,
        }) = &self.action
        {}
        crate_options.save_to_file(&self.config_path, &self.config_name);
    }
    pub fn create_snippet_action(&self) {
        if let Some(Actions::CreateSnippet { path_to_snippets }) = &self.action {
            match helpers::create_snippets(path_to_snippets) {
                (Some(_), Some(_)) => println!("snippets crate success in {}", &path_to_snippets),
                _ => eprintln!("someting went wrong. dir {}", &path_to_snippets),
            }
        }
    }
    pub fn do_action(&self) -> anyhow::Result<()> {
        let crate_options = CrateOptions::load_from_file(&self.config_path, &self.config_name);
        if let Some(Actions::Do {
            dir_path,
            name,
            is_lib: _,
            add_link: _,
            build,
            crates,
        }) = &self.action
        {
            helpers::check_and_create_dir(dir_path)?;
            let mut com = std::process::Command::new("cargo")
                .args(["new", "--bin", name])
                .current_dir(dir_path)
                .spawn()?;
            if !com.wait()?.success() {
                return Err(anyhow!("create crate {} error in {}", &name, &dir_path));
            }
            if let Some(crates) = crates {
                let crates = crates
                    .iter()
                    .filter_map(|c| crate_options.get_crate(c).map(|co| Crate::from(co.clone())))
                    .collect::<Vec<Crate>>();

                for c in crates.iter() {
                    c.add_crate(dir_path)?;
                    c.add_use(dir_path)?;
                }
                for c in crates.iter() {
                    c.add_core(dir_path)?;
                }
            }
            if *build {
                let mut com = std::process::Command::new("cargo")
                    .arg("build")
                    .current_dir(dir_path)
                    .spawn()?;
                if com.wait()?.success() {
                    println!("{} built", &name);
                }
            }
        }
        Ok(())
    }
}
