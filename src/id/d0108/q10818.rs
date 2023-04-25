// BOJ 10818 [Max and Min]
// Supported by GitHub Copilot

use std::io::{self, BufRead, Write};

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    si.read_line(&mut String::new()).unwrap();
    let mut input = String::new();
    si.read_line(&mut input).unwrap();

    let mut mx = -1<<20;
    let mut mn = 1<<20;
    for x in input.split_whitespace() {
        let x = x.parse::<i32>().unwrap();
        mx = mx.max(x);
        mn = mn.min(x);
    }

    writeln!(so, "{} {}", mn, mx).unwrap();
}
