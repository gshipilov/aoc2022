use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn read_input_lines(name: &str) -> Vec<String> {
    read_input(name).lines().map(String::from).collect()
}

pub fn read_input(name: &str) -> String {
    let mut path = PathBuf::new();
    path.push("inputs");
    path.push(name);

    let mut file = File::open(&path)
        .unwrap_or_else(|_| panic!("File {} should exist", path.to_string_lossy()));

    let mut out = String::new();
    file.read_to_string(&mut out).unwrap();
    out
}
