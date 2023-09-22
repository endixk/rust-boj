// BOJ 24370 [Asymptotic Notation 6]
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

fn lb(a: i32, b: i32, c: i32, k: i32, n: i32) -> bool {
    return if a > k { false }
    else if a == k {
        if b > 0 { false }
        else if b == 0 { c <= 0 }
        else { -c as f64 / b as f64 <= n as f64 }
    } else {
        let det = b*b - 4*(a-k)*c;
        if det <= 0 { true }
        else {
            (-b as f64 - (det as f64).sqrt()) / (2.0 * (a-k) as f64) <= n as f64
        }
    }
}
fn rb(a: i32, b: i32, c: i32, k: i32, n: i32) -> bool {
    return if a < k { false }
    else if a == k {
        if b < 0 { false }
        else if b == 0 { c >= 0 }
        else { -c as f64 / b as f64 <= n as f64 }
    } else {
        let det = b*b - 4*(a-k)*c;
        if det <= 0 { true }
        else {
            (-b as f64 + (det as f64).sqrt()) / (2.0 * (a-k) as f64) <= n as f64
        }
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (a, b, c, d, e, n) = (
        next::<i32>(&mut it), next::<i32>(&mut it), next::<i32>(&mut it),
        next::<i32>(&mut it), next::<i32>(&mut it), next::<i32>(&mut it)
    );
    if rb(a, b, c, d, n) && lb(a, b, c, e, n) { writeln!(so, "1")?; }
    else { writeln!(so, "0")?; }

    Ok(())
}
