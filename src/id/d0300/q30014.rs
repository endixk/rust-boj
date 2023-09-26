// BOJ 30014 [Love]
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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut p = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    p.sort_unstable();

    let mut q = std::collections::VecDeque::new();
    let mut f = true;
    for a in p.into_iter().rev() {
        if f { q.push_back(a); } else { q.push_front(a); }
        f = !f;
    }

    let mut ans = q[0] * q.iter().last().unwrap();
    for i in 1..q.len() {
        ans += q[i] * q[i - 1];
    }
    writeln!(so, "{}", ans)?;
    q.into_iter().for_each(|a| write!(so, "{} ", a).unwrap());
    writeln!(so)?;

    Ok(())
}
