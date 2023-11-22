// BOJ 30647 [Standings Management]
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

#[inline] fn parse(s: String) -> (i32, String, bool) {
    // split s with quotes, commas, and colons
    let v = s.split(|c| "\",:}".contains(c)).collect::<Vec<&str>>();
    (-v[9].parse::<i32>().unwrap(), v[4].into(), v[13].parse::<u8>().unwrap() == 1)
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut v = vec![];
    for _ in 0..n {
        v.push(parse(next::<String>(&mut it)));
    }
    v.sort_unstable();

    let (mut x, mut r) = (v[0].0, 1);
    for i in 0..n {
        if x != v[i].0 {
            x = v[i].0;
            r = i+1;
        }
        if !v[i].2 {
            writeln!(so, "{} {} {}", r, v[i].1, -v[i].0)?;
        }
    }

    Ok(())
}
