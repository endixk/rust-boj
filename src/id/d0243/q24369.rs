// BOJ 24369 [Asymptotic Notation 5]
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

    let (a, b, c, d, n) = (
        next::<i32>(&mut it), next::<i32>(&mut it), next::<i32>(&mut it),
        next::<i32>(&mut it), next::<i32>(&mut it)
    );

    if a < d { writeln!(so, "0")?; }
    else if a == d {
        if b < 0 { writeln!(so, "0")?; }
        else if b == 0 { writeln!(so, "{}", if c >= 0 { "1" } else { "0 "})?; }
        else { writeln!(so, "{}", if -c as f64 / b as f64 <= n as f64 { "1" } else { "0 "})?; }
    } else {
        let det = b*b - 4*(a-d)*c;
        if det <= 0 { writeln!(so, "1")?; }
        else {
            let x2 = (-b as f64 + (det as f64).sqrt()) / (2.0 * (a-d) as f64);
            writeln!(so, "{}", if x2 <= n as f64 { "1" } else { "0 "})?;
        }
    }

    Ok(())
}
