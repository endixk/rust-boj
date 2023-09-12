// BOJ 12789 [Snack Time]
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

    let n = next::<usize>(&mut it);
    let mut a = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    a.reverse();
    let mut b = Vec::new();
    let mut c = 1;
    loop {
        if a.is_empty() {
            if b.is_empty() { break; }
            let x = b.pop().unwrap();
            if x != c { writeln!(so, "Sad").unwrap(); return; }
            c += 1;
        } else {
            if b.is_empty() {
                let x = a.pop().unwrap();
                if x == c { c += 1; }
                else { b.push(x); }
            } else {
                let x = b.pop().unwrap();
                if x == c { c += 1; }
                else {
                    b.push(x);
                    let x = a.pop().unwrap();
                    if x == c { c += 1; }
                    else { b.push(x); }
                }
            }
        }
    }
    writeln!(so, "Nice").unwrap();
}
