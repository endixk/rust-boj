// BOJ 3779 [Period]
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

fn pi(s: &[u8]) -> Vec<usize> {
    let l = s.len();
    let mut p = vec![0; l];

    let mut j = 0;
    for i in 1..l {
        while j > 0 && s[i] != s[j] { j = p[j-1]; }
        if s[i] == s[j] { j += 1; p[i] = j; }
    }

    p
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut tc = 1;
    loop {
        match next::<usize>(&mut it) {
            0 => { break; },
            _ => { writeln!(so, "Test case #{}", tc).unwrap(); tc += 1; }
        }

        let s = next::<String>(&mut it);
        let pi = pi(s.as_bytes());
        for (i, &p) in pi.iter().enumerate() {
            let k = if (i+1) % (i+1-p) == 0 { (i+1)/(i+1-p) } else { 1 };
            if k > 1 { writeln!(so, "{} {}", i+1, k).unwrap(); }
        }
        writeln!(so).unwrap();
    }
}
