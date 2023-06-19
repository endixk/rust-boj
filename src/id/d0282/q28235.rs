// BOJ 28235 [CodeMaster 2023]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

pub fn main() {
    let s = io::stdin().lock().lines().next().unwrap().unwrap();
    match s.as_str() {
        "SONGDO" => println!("HIGHSCHOOL"),
        "CODE" => println!("MASTER"),
        "2023" => println!("0611"),
        "ALGORITHM" => println!("CONTEST"),
        _ => unreachable!(),
    }
}

