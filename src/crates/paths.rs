pub struct Paths {
    pub dir: String,
    pub main: String,
    pub config: String,
    pub config_file: String,
}
impl Paths {
    fn new(dir: String, config: String, config_file: String) -> Self {
        let main = format!("{}/src/main.rs", &dir);
        Self {
            dir,
            main,
            config,
            config_file,
        }
    }
}
