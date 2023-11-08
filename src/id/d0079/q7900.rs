// BOJ 7900 [Hansel and Grethel]
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

    for _ in 0..next(&mut it) {
        let (x1, y1, d1) = (next::<i32>(&mut it), next::<i32>(&mut it), next::<i32>(&mut it));
        let (x2, y2, d2) = (next::<i32>(&mut it), next::<i32>(&mut it), next::<i32>(&mut it));
        let (d1, d2) = (d1 % 180, d2 % 180);
        if d1 == 0 {
            let m = (90.0 - d2 as f64).to_radians().tan();
            writeln!(so, "{:.4} {:.4}", x1 as f64, y2 as f64 + (x1 - x2) as f64 * m)?;
        } else if d2 == 0 {
            let m = (90.0 - d1 as f64).to_radians().tan();
            writeln!(so, "{:.4} {:.4}", x2 as f64, y1 as f64 + (x2 - x1) as f64 * m)?;
        } else {
            let m1 = (90.0 - d1 as f64).to_radians().tan();
            let m2 = (90.0 - d2 as f64).to_radians().tan();
            let x = (y1 as f64 - y2 as f64 + x2 as f64 * m2 - x1 as f64 * m1) / (m2 - m1);
            writeln!(so, "{:.4} {:.4}", x, y1 as f64 + (x - x1 as f64) * m1)?;
        }
    }

    Ok(())
}
