// BOJ 1874 [Stack Sequence]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
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
    let n: usize = next(&mut it);
    let seq: Vec<usize> = (0..n).map(|_| next(&mut it)).collect();
    let mut si = 0;

    let mut st = Vec::with_capacity(n);
    let mut cmd = String::new();
    for i in 1..=n {
        st.push(i); cmd += "+ ";
        while let Some(&top) = st.last() {
            if top == seq[si] {
                st.pop(); cmd += "- "; si += 1;
            } else {
                break;
            }
        }
    }

    if st.is_empty() {
        writeln!(so, "{}", cmd).unwrap();
    } else {
        writeln!(so, "NO").unwrap();
    }
}
