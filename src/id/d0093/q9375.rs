// BOJ 9375 [Incognito]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let mut map = HashMap::new();
        for _ in 0..next(&mut it) {
            let (_, key) = (next::<String>(&mut it), next::<String>(&mut it));
            *map.entry(key).or_insert(0) += 1;
        }

        let mut ans = 1;
        for (_, v) in map {
            ans *= v + 1;
        }
        writeln!(so, "{}", ans - 1).unwrap();
    }
}
