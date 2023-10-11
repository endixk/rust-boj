// BOJ 7869 [Circular Area]
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

    let (x1, y1, r1, x2, y2, r2) = (
        next::<f64>(&mut it), next::<f64>(&mut it), next::<f64>(&mut it),
        next::<f64>(&mut it), next::<f64>(&mut it), next::<f64>(&mut it)
    );
    let (xs, xe) = (
        (x1 - r1).min(x2 - r2) - 1.0,
        (x1 + r1).max(x2 + r2) + 1.0
    );

    let n = 100000;
    let dx = (xe - xs) / n as f64;
    let mut x = xs;
    let mut ans = 0.0;
    for _ in 0..n {
        x += dx;

        let d1 = r1.powi(2) - (x - x1).powi(2);
        if d1 < 1e-9 { continue; }
        let d2 = r2.powi(2) - (x - x2).powi(2);
        if d2 < 1e-9 { continue; }

        if y1 + d1.sqrt() < y2 - d2.sqrt() { continue; }
        if y1 - d1.sqrt() > y2 + d2.sqrt() { continue; }

        let ya = (y1 + d1.sqrt()).min(y2 + d2.sqrt());
        let yb = (y1 - d1.sqrt()).max(y2 - d2.sqrt());
        ans += dx * (ya - yb).abs();
    }

    writeln!(so, "{:.3}", ans)?;

    Ok(())
}
