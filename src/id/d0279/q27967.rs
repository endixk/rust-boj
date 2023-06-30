// BOJ 27967 [Masked parentheses]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

fn go(s: &[u8], i: usize, k: i32, r: &String) {
    if k < 0 { return; }
    if i == s.len() {
        if k == 0 {
            println!("{}", r);
            std::process::exit(0);
        } else {
            return;
        }
    }

    match s[i] {
        b'G' => {
            go(s, i + 1, k + 1, &format!("{}(", r));
            go(s, i + 1, k - 1, &format!("{})", r));
        },
        b'(' => {
            go(s, i + 1, k + 1, &format!("{}(", r));
        },
        _ => {
            go(s, i + 1, k - 1, &format!("{})", r));
        }
    }
}

pub fn main() {
    let s = io::stdin().lock().lines().skip(1).next().unwrap().unwrap();
    let s = s.as_bytes();
    go(s, 0, 0, &String::new());
}
