// BOJ 2560 [Paramecia]
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

#[inline] fn sum(p: &[i16], i: i32, j: i32) -> i16 {
    if j < 0 { return 0; }
    (p[j as usize] - if i < 0 { 0 } else { p[i as usize] } + 1000) % 1000
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (a, b, d, n) = (
        next::<i32>(&mut it), next::<i32>(&mut it),
        next::<i32>(&mut it), next::<usize>(&mut it));
    let mut p = vec![0; n+1];
    p[0] = 1;
    for i in 1..=n {
        p[i] = (p[i-1] + sum(&p, i as i32 - b, i as i32 - a)) % 1000;
    }
    writeln!(so, "{}", sum(&p, n as i32 - d, n as i32)).ok();

    Ok(())
}
