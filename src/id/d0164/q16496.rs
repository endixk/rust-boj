// BOJ 16496 [Biggest Number]
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
    let mut flag = false;
    let mut v = (0..n).map(|_| {
        let s = next::<String>(&mut it);
        flag |= s != "0";
        (s.parse::<u64>().unwrap(), s.len() as u32)
    }).collect::<Vec<_>>();
    if !flag { writeln!(so, "0").unwrap(); return; }

    v.sort_unstable_by(|&(a, al), &(b, bl)| {
        (b * 10u64.pow(al) + a).cmp(&(a * 10u64.pow(bl) + b))
    });
    for (n, _) in v { write!(so, "{}", n).unwrap(); }
}
