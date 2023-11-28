// BOJ 30685 [Melting Butter]
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
    let mut b = (0..n).map(|_| (next::<i64>(&mut it), next::<i64>(&mut it))).collect::<Vec<_>>();
    b.sort_unstable();

    const INF: i64 = 0x3f3f3f3f3f3f3f3f;
    let mut ans = INF;
    for i in 1..n {
        let d = b[i].0 - b[i-1].0 - 1;
        if b[i].1/2 + b[i-1].1/2 <= d { continue; }
        if b[i].1/2 > d/2 && b[i-1].1/2 > d/2 {
            ans = ans.min(d/2);
        } else if b[i].1 > b[i-1].1 {
            ans = ans.min(d - b[i-1].1/2);
        } else {
            ans = ans.min(d - b[i].1/2);
        }
    }
    if ans == INF { writeln!(so, "forever")?; }
    else { writeln!(so, "{}", ans)?; }

    Ok(())
}
