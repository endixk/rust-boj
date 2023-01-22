// BOJ 2164 [Card2]
// Supported by GitHub Copilot

use std::collections::VecDeque;
use std::io::{self, BufRead, Write};

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    let n = {
        si.read_line(&mut buf).unwrap();
        buf.trim().parse::<i32>().unwrap()
    };

    let mut q: VecDeque<i32> = (1..=n).collect();
    while q.len() > 1 {
        q.pop_front();
        let x = q.pop_front().unwrap();
        q.push_back(x);
    }
    writeln!(so, "{}", q.pop_front().unwrap()).unwrap();
}
