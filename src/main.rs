use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use regex::Regex;
use chrono::{Days, NaiveDate};

const DEFAULT_DIR_PATH: &str = "/home/carlo/Desktop";
const FILE_NAME_PATTERN: &str = "^Todos-w[0-9]{8}\\.md$";
const FILE_CONTENT_PATTERN: &str = r"^# Todos week [0-9]{8}\n"; // simplified file content check
const FILE_DELETION_PATTERN: &str = r"- \[x\]";

fn main() {
    // parse command line arguments
    let args: Vec<String> = env::args().collect();
    let dir_path = match args.get(1) {
        None => {DEFAULT_DIR_PATH}
        Some(dir) => {dir}
    };

    // explore the directory to find the most recent weekly plan
    let latest_week_file_path = find_latest_weekly_plan(dir_path).expect("Couldn't find weekly plan file in directory");
    let latest_week_file_path = latest_week_file_path.as_path();
    println!("Found latest week plan in provided directory: {:?}", latest_week_file_path);

    // check if the most recent weekly plan has the correct content
    if !file_matches_format(&latest_week_file_path) {
        println!("File content doesn't match the expected pattern {FILE_CONTENT_PATTERN}");
        return;
    }

    // create next week's plan from previous week's
    let next_week_file_content = create_content_from_prev_week(latest_week_file_path);

    // create next week's plan file pathname from previous week's
    let next_week_file_path = create_path_from_prev_week(latest_week_file_path);

    // write next week's plan content to the file
    println!("Writing next week's plan into: {next_week_file_path}");
    fs::write(next_week_file_path, next_week_file_content).expect("Couldn't write to file!");
    println!("Done!");
}

fn find_latest_weekly_plan(dir_path: &str) -> Option<PathBuf> {
    let regex = Regex::new(FILE_NAME_PATTERN).unwrap();

    let latest_week_file_path = fs::read_dir(dir_path).ok()?
        .filter(|f|
            regex.is_match(&f.as_ref().unwrap().file_name().into_string().unwrap()))
        .max_by(|x, y|
            x.as_ref().unwrap().file_name().cmp(&y.as_ref().unwrap().file_name()))?.ok()?
        .path();

    Some(latest_week_file_path)
}

fn file_matches_format(path: &Path) -> bool {
    let latest_week_file_content = fs::read_to_string(path).expect("Couldn't read previous week's file!");
    let regex = Regex::new(FILE_CONTENT_PATTERN).unwrap();

    regex.is_match(&latest_week_file_content)
}

fn create_content_from_prev_week(path: &Path) -> String {
    let latest_week_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .expect("Couldn't open previous week's file");

    let regex = Regex::new(FILE_DELETION_PATTERN).unwrap();
    BufReader::new(latest_week_file)
        .lines()
        .map(|x| x.unwrap())
        .filter(|l| !regex.is_match(&l))
        .collect::<Vec<String>>()
        .join("\n")
}

fn create_path_from_prev_week(path: &Path) -> String {
    let next_week_file_path = path.to_str().unwrap();
    let latest_week_date = next_week_file_path.split_at(next_week_file_path.len() - 11).1.split_at(8).0;
    let next_week_date = NaiveDate::parse_from_str(latest_week_date, "%Y%m%d")
        .unwrap()
        .checked_add_days(Days::new(7))
        .unwrap();
    let next_week_date = next_week_date.to_string().replace("-", "");

    next_week_file_path.split_at(next_week_file_path.len() - 11).0.to_owned() + &next_week_date.to_string() + ".md"
}