// BOJ 14612 [Restaurant]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, _) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut v = Vec::new();
    for _ in 0..n {
        let cmd = next::<String>(&mut it);
        match cmd.chars().next().unwrap() {
            'o' => {
                let (n, t) = (next::<usize>(&mut it), next::<usize>(&mut it));
                v.push((t, n));
            },
            's' => v.sort_unstable(),
            _ => {
                let n = next::<usize>(&mut it);
                let i = v.iter().position(|&x| x.1 == n).unwrap();
                v.remove(i);
            }
        }
        if v.is_empty() {
            writeln!(so, "sleep").unwrap();
        } else {
            for x in &v {
                write!(so, "{} ", x.1).unwrap();
            }
            writeln!(so).unwrap();
        }
    }
}
