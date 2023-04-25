// BOJ 10816 [Number Cards 2]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    read(&mut si);
    let mut map = HashMap::new();
    for c in read(&mut si)
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>(){
        *map.entry(c).or_insert(0) += 1;
    }

    read(&mut si);
    for v in read(&mut si)
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>(){
        write!(so, "{} ", map.get(&v).unwrap_or(&0)).unwrap();
    }
    writeln!(so).unwrap();
}
