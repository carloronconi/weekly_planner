use std::env;
use std::fs;
use regex::Regex;

fn main() {
    const DEFAULT_DIR_PATH: &str = "/home/carlo/Desktop";
    const FILE_NAME_PATTERN: &str = "^Todos-w[0-9]{8}\\.md$";
    const FILE_CONTENT_PATTERN: &str = r"^# Todos week [0-9]{8}\n
## Uni
((\s)*- \[[ |x]\] .*
)*
## Misc
((\s)*- \[[ |x]\] .*
)*
## Other
((\s)*- \[[ |x]\] .*
)*
---";

    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let regex = Regex::new(FILE_NAME_PATTERN).unwrap();

    let dir_path = match args.get(1) {
        None => {DEFAULT_DIR_PATH}
        Some(dir) => {dir}
    };

    let latest_week_file = fs::read_dir(dir_path)
        .expect(&format!("Couldn't read the directory {dir_path}"))
        .filter(|f|
            regex.is_match(&f.as_ref().unwrap().file_name().into_string().unwrap()))
        .max_by(|x, y|
            x.as_ref().unwrap().file_name().cmp(&y.as_ref().unwrap().file_name()))
        .unwrap()
        .unwrap()
        .path();

    dbg!(&latest_week_file);

    let latest_week_file = fs::read_to_string(latest_week_file)
        .expect("Couldn't read the file!");

    dbg!(&latest_week_file);

    let regex = Regex::new(FILE_CONTENT_PATTERN).unwrap();
    if !regex.is_match(&latest_week_file) { panic!("File content doesn't match the expected pattern {FILE_CONTENT_PATTERN}"); }

    // TODO: delete lines starting with "- [x]"
}
