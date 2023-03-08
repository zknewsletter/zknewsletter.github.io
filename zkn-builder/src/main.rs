use std::{fs, path::PathBuf};

fn main() {
    let issue = fs::read_to_string(PathBuf::from("../issues/2023-03-08.md")).unwrap();
    println!("{}", markdown::to_html(&issue));
}
