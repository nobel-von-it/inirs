pub fn check_and_create_dir(path: &str) -> anyhow::Result<()> {
    if !std::path::Path::new(path).exists() {
        std::fs::create_dir_all(path)?
    }
    Ok(())
}
pub fn check_and_create_file(path: &str) -> anyhow::Result<()> {
    if !std::path::Path::new(path).exists() {
        std::fs::File::create(path)?;
    }
    Ok(())
}
pub fn get_crate_link(s: &str) -> String {
    format!("https://docs.rs/{s}")
}
pub fn create_snippets(path: &str) -> (Option<String>, Option<String>) {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };
    let file =
        File::open(path).unwrap_or(File::open(format!("{}/src/main.rs", path)).unwrap_or(
            File::open(format!("{}/scr/lib.rs", path)).unwrap_or_else(|e| panic!("{e}")),
        ));
    let reader = BufReader::new(file);

    let mut imps = vec![];
    let mut code_body = vec![];

    let mut found_code = false;
    let mut in_imp_block = false;

    reader.lines().for_each(|l| {
        let l = l.unwrap();
        let tl = l.trim();

        if found_code {
            code_body.push(l.clone())
        } else if in_imp_block {
            imps.push(l.clone());
            if tl.ends_with("};") {
                in_imp_block = false;
            }
        } else if tl.starts_with("use") {
            imps.push(l.clone());
            if tl.ends_with('{') {
                in_imp_block = true;
            }
        } else {
            code_body.push(l.clone());
            found_code = true;
        }
    });

    (
        if imps.is_empty() {
            None
        } else {
            Some(imps.join("\n"))
        },
        if code_body.is_empty() {
            None
        } else {
            Some(code_body.join("\n"))
        },
    )
}
