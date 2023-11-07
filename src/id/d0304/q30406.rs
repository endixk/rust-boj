// BOJ 30406 [Santa's Gift]
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

    let mut a = [0, 0, 0, 0];
    for _ in 0..next(&mut it){ a[next::<usize>(&mut it)] += 1; }

    let mut ans = 0;
    let (x, y) = (a[0].min(a[3]), a[1].min(a[2]));
    a[0] -= x; a[1] -= y; a[2] -= y; a[3] -= x;
    ans += (x + y) * 3;

    let (x, y) = (a[0].min(a[2]), a[1].min(a[3]));
    a[0] -= x; a[1] -= y; a[2] -= x; a[3] -= y;
    ans += (x + y) * 2;

    ans += a[0].min(a[1]) + a[2].min(a[3]);
    writeln!(so, "{}", ans)?;

    Ok(())
}
