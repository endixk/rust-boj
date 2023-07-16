// BOJ 1783 [Sick Knight]
// Supported by GitHub Copilot

use std::io::{self, BufRead};
pub fn main() {
    let v = io::stdin()
        .lock().lines().next().unwrap().unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let (n, m) = (v[0], v[1]);

    if n == 1 { println!("1"); }
    else if n == 2 { println!("{}", if m < 7 { (m + 1) / 2 } else { 4 }); }
    else if m < 7 { println!("{}", if m < 4 { m } else { 4 }); }
    else { println!("{}", m - 2); }
}
