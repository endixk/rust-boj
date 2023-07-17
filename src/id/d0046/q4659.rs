// BOJ 4659 [Easier Done than Said?]
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

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
fn rule_1(s: &String) -> bool {
    s.chars().filter(|&c| VOWELS.contains(&c)).count() > 0
}
fn rule_2(s: &String) -> bool {
    let v = s.chars().map(|c| if VOWELS.contains(&c) { 1 } else { 0 }).collect::<Vec<_>>();
    for i in 2..s.len() {
        match v[i-2] + v[i-1] + v[i] {
            0 => return false,
            3 => return false,
            _ => continue,
        }
    }
    true
}
fn rule_3(s: &String) -> bool {
    let mut c = s.chars().next().unwrap();
    for x in s.chars().skip(1) {
        if c == x && c != 'e' && c != 'o' {
            return false;
        }
        c = x;
    }
    true
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    loop {
        match next::<String>(&mut it) {
            s if s == "end" => break,
            s => writeln!(so, "<{}> is {}.", s,
                          if rule_1(&s) && rule_2(&s) && rule_3(&s) { "acceptable" }
                          else { "not acceptable" }).unwrap(),
        }
    }
}
