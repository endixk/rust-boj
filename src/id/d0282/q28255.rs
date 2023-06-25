// BOJ 28255 [Chocolate Ice Cream]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};

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

fn pref(s: &str) -> String {
    let n = s.len();
    let n = if n % 3 == 0 { n / 3 } else { n / 3 + 1 };
    s[..n].to_string()
}

fn rev(s: &String) -> String {
    s.clone().chars().rev().collect::<String>()
}

fn tail(s: &String) -> String {
    s.clone().chars().skip(1).collect::<String>()
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let s = next::<String>(&mut it);
        let p = pref(&s);
        if p.clone() + &rev(&p) + &p.clone() == s {
            writeln!(so, "1").ok();
        } else if p.clone() + &tail(&rev(&p)) + &p.clone() == s {
            writeln!(so, "1").ok();
        } else if p.clone() + &rev(&p) + &tail(&p) == s {
            writeln!(so, "1").ok();
        } else if p.clone() + &tail(&rev(&p)) + &tail(&p) == s {
            writeln!(so, "1").ok();
        } else {
            writeln!(so, "0").ok();
        }
    }
}