// BOJ 30640 [Driving Practice]
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
    let (mut a, mut f) = (vec![], vec![]);
    for _ in 0..=n {
        a.push(next::<i64>(&mut it));
        f.push(next::<i64>(&mut it));
    }

    let mut v = vec![0; n+1];
    for i in (1..=n).rev() {
        v[i-1] += v[i] + f[i-1] + a[i-1] - a[i];
    }

    let (mut x, mut k) = (v[0], 0);
    let mut h = 0;
    for i in 1..=n {
        h += f[i-1] + a[i-1] - a[i];
        if h < 0 {
            for _ in i..=n {
                writeln!(so, "-1 -1")?;
            }
            break;
        }
        writeln!(so, "{} {}", k, x - v[i])?;
        if x <= v[i] { x = v[i]; k = i; }
    }

    Ok(())
}
