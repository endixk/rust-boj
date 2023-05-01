// BOJ 10773 [Zero That Out]
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

    let mut st = Vec::new();
    (0..next(&mut it)).for_each(|_| {
        let n = next::<i32>(&mut it);
        if n == 0 && !st.is_empty() { st.pop(); }
        else { st.push(n) }
    });

    writeln!(so, "{}", st.iter().sum::<i32>()).unwrap();
}
