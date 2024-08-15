pub struct CratesInfo {
    pub paths: Paths,
    pub crates: Vec<Crate>,
    pub is_cross: bool,
    pub is_rat: bool,
    pub is_tokio: bool,
}
impl CratesInfo {
    // pub fn new(args: &[String], path: String) -> Self {
    //     let args = Args::parse();
    //     let mut is_cross = false;
    //     let mut is_rat = false;
    //     let mut is_tokio = false;
    //
    //     let crate_options = CrateOptions::load_from_file(jkk, config_name)
    //     let crates = args
    //         .iter()
    //         .map(|c| {
    //             let cl = c.to_lowercase();
    //             if cl == "ratatui" {
    //                 is_rat = true;
    //             }
    //             if cl == "crossterm" {
    //                 is_cross = true;
    //             }
    //             if cl == "tokio" {
    //                 is_tokio = true;
    //             }
    //             Crate::from(CrateOptions::get_crate(&self, name))
    //         })
    //         .collect::<Vec<_>>();
    //     Self {
    //         path_to_crate: path,
    //         path_to_main,
    //         crates,
    //         is_cross,
    //         is_rat,
    //         is_tokio,
    //     }
    // }
}
