// BOJ 9012 [Parentheses]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let buf = read(&mut si);
    for _ in 0..buf.trim().parse().unwrap() {
        let buf = read(&mut si);
        let mut depth = 0;
        for c in buf.chars() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                _ => (),
            }
            if depth < 0 { break }
        }

        if depth == 0 {
            writeln!(so, "YES").unwrap();
        } else {
            writeln!(so, "NO").unwrap();
        }
    }
}
