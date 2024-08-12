use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CrateOption {
    pub name: String,
    pub link: Option<String>,
    pub features: Option<Vec<String>>,
    pub path_to_snippets: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct CrateOptions {
    crate_options: Vec<CrateOption>,
}
impl CrateOptions {
    pub fn get_file(config_path: &str, config_name: &str) -> std::fs::File {
        let file_path = format!("{}{}", config_path, config_name);
        std::fs::File::open(file_path.clone()).unwrap_or_else(|_| {
            let _ = std::fs::create_dir_all(config_path.clone());
            let _ = std::fs::File::create(file_path.clone());
            std::fs::File::open(file_path).unwrap()
        })
    }
    pub fn load_from_file(config_path: &str, config_name: &str) -> Self {
        let mut file = CrateOptions::get_file(config_path, config_name);
        serde_json::from_reader(&mut file).unwrap_or(CrateOptions {
            crate_options: vec![],
        })
    }
    pub fn save_to_file(&self, config_path: &str, config_name: &str) {
        let mut file = CrateOptions::get_file(config_path, config_name);
        serde_json::to_writer(&mut file, self).unwrap();
    }
    pub fn get_crate(&self, name: &str) -> Option<&CrateOption> {
        self.crate_options
            .iter()
            .find(|co| co.name.to_lowercase() == name.to_lowercase())
    }
    pub fn add_crate(&mut self, co: CrateOption) {
        self.crate_options.push(co)
    }
}
