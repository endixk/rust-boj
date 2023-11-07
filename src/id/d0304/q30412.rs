// BOJ 30412 [Cat Tower]
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

fn get(a: i64, b: i64, c: i64, x: i64) -> i64 {
    let (mut p, q, mut k) = (b, c, 0);
    if (a - p).abs() < x {
        k += a + x - p; p = a + x;
    }
    if (p - q).abs() < x {
        if p < q {
            k += p + x - q;
        } else {
            k += q + x - p; p = q + x;
        }
    }
    if (a - p).abs() < x {
        k += a + x - p;
    }
    k
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, x) = (next::<usize>(&mut it), next::<i64>(&mut it));
    let mut a = vec![-x; n+2];
    for i in 1..=n {
        a[i] = next::<i64>(&mut it);
    }

    let mut ans = i64::MAX;
    for i in 1..=n {
        let mut k = 0;
        if (a[i-1] - a[i]).abs() < x { k += a[i] + x - a[i-1]; }
        if (a[i] - a[i+1]).abs() < x { k += a[i] + x - a[i+1]; }
        ans = ans.min(k);
        ans = ans.min(get(a[i-1], a[i], a[i+1], x));
        ans = ans.min(get(a[i+1], a[i], a[i-1], x));
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
