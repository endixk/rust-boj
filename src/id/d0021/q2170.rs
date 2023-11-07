// BOJ 2170 [Drawing Lines]
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
    let mut v = Vec::new();
    for _ in 0..n {
        v.push((next::<i32>(&mut it), true));
        v.push((next::<i32>(&mut it), false));
    }
    v.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let (mut ans, mut prv, mut cnt) = (0, i32::MIN, 0);
    for (x, f) in v {
        if cnt > 0 { ans += x - prv; }
        if f { cnt += 1; } else { cnt -= 1; }
        prv = x;
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
