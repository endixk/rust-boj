// BOJ 10814 [Sort by Age]
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

    let mut v: Vec<String> = Vec::with_capacity(201);
    for _ in 0..201 {
        v.push(String::new());
    }
    for _ in 0..read(&mut si).parse().unwrap() {
        let buf = read(&mut si);
        let mut s = buf.split_whitespace();
        v[s.next().unwrap().parse::<usize>().unwrap()] += format!(",{}", s.next().unwrap()).as_str();
    }

    for (age, names) in v.iter().enumerate() {
        for name in names.split(',').skip(1) {
            writeln!(so, "{} {}", age, name).unwrap();
        }
    }
}
