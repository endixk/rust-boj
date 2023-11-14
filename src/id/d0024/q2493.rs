// BOJ 2493 [Tower]
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
    let t = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();

    let mut v = vec![(0x3f3f3f3f, 0)];
    for i in 0..n {
        while let Some(x) = v.pop() {
            if x.0 > t[i] {
                v.push(x);
                write!(so, "{} ", x.1)?;
                break;
            }
        }
        v.push((t[i], i + 1));
    }
    writeln!(so)?;

    Ok(())
}
