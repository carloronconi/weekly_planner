use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use regex::Regex;
use chrono::{Days, NaiveDate};

fn main() {
    const DEFAULT_DIR_PATH: &str = "/home/carlo/Desktop";
    const FILE_NAME_PATTERN: &str = "^Todos-w[0-9]{8}\\.md$";
    const FILE_CONTENT_PATTERN: &str = r"^# Todos week [0-9]{8}\n"; // simplified file content check
    const FILE_DELETION_PATTERN: &str = r"- \[x\]";

    let args: Vec<String> = env::args().collect();

    let regex = Regex::new(FILE_NAME_PATTERN).unwrap();

    let dir_path = match args.get(1) {
        None => {DEFAULT_DIR_PATH}
        Some(dir) => {dir}
    };

    // explore the directory to find the most recent weekly plan
    let latest_week_file_path = fs::read_dir(dir_path)
        .expect(&format!("Couldn't read the directory {dir_path}"))
        .filter(|f|
            regex.is_match(&f.as_ref().unwrap().file_name().into_string().unwrap()))
        .max_by(|x, y|
            x.as_ref().unwrap().file_name().cmp(&y.as_ref().unwrap().file_name()))
        .unwrap()
        .unwrap()
        .path();

    println!("Found latest week plan in provided directory: {}", latest_week_file_path.as_path().to_str().unwrap());

    // check if the most recent weekly plan has the correct content
    let latest_week_file_content = fs::read_to_string(&latest_week_file_path)
        .expect("Couldn't read the file!");

    let regex = Regex::new(FILE_CONTENT_PATTERN).unwrap();
    if !regex.is_match(&latest_week_file_content) { panic!("File content doesn't match the expected pattern {FILE_CONTENT_PATTERN}"); }

    // create next week's plan from previous week's
    let latest_week_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(latest_week_file_path.as_path())
        .unwrap();

    let regex = Regex::new(FILE_DELETION_PATTERN).unwrap();
    let next_week_file_content = BufReader::new(latest_week_file)
        .lines()
        .map(|x| x.unwrap())
        .filter(|l| !regex.is_match(&l))
        .collect::<Vec<String>>()
        .join("\n");

    // create next week's plan file name from previous week's
    let next_week_file_path = latest_week_file_path.into_os_string().into_string().unwrap();

    let latest_week_date = next_week_file_path.split_at(next_week_file_path.len() - 11).1.split_at(8).0;

    let next_week_date = NaiveDate::parse_from_str(latest_week_date, "%Y%m%d")
        .unwrap()
        .checked_add_days(Days::new(7))
        .unwrap();
    let next_week_date = next_week_date.to_string().replace("-", "");

    let next_week_file_path = next_week_file_path.split_at(next_week_file_path.len() - 11).0.to_owned() + &next_week_date.to_string() + ".md";

    // write next week's plan content to the file
    println!("Writing next week's plan into: {next_week_file_path}");
    fs::write(next_week_file_path, next_week_file_content).expect("Couldn't write to file!");

    println!("Done!");
}
