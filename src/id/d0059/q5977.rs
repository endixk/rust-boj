// BOJ 5977 [Mowing the Lawn]
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

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let a = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();

    let mut q = std::collections::VecDeque::<(usize, usize)>::new();
    for i in 0..n {
        let x = if i <= k { 0 } else { q.front().unwrap().0 };
        while !q.is_empty() && q.front().unwrap().1 + k < i { q.pop_front(); }
        while !q.is_empty() && q.back().unwrap().0 >= a[i] + x { q.pop_back(); }
        q.push_back((a[i] + x, i));
    }

    writeln!(so, "{}", a.iter().sum::<usize>() - q.front().unwrap().0)?;

    Ok(())
}
