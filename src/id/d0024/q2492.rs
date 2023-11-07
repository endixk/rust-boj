// BOJ 2492 [Jewels]
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

fn get(i: i32, j: i32, k: i32, a: &[i32], b: &[i32]) -> i32 {
    let mut c = 0;
    for (&x, &y) in a.iter().zip(b.iter()) {
        if i <= x && x <= i + k && j - k <= y && y <= j {
            c += 1;
        }
    }
    c
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, _, t, k) = (next::<i32>(&mut it), next::<i32>(&mut it), next::<usize>(&mut it), next::<i32>(&mut it));
    let (mut a, mut b) = (vec![], vec![]);
    for _ in 0..t {
        a.push(next::<i32>(&mut it));
        b.push(next::<i32>(&mut it));
    }

    let (mut c, mut x, mut y) = (0, 0, k);
    for i in 0..t {
        for j in i..t {
            let p = if a[i] < a[j] { a[i] } else { a[j] };
            let q = if b[i] > b[j] { b[i] } else { b[j] };
            let p = if p + k > n { n - k } else { p };
            let q = if q - k < 0 { k } else { q };
            let z = get(p, q, k, &a, &b);
            if c < z { c = z; x = p; y = q; }
        }
    }
    writeln!(so, "{} {} {}", x, y, c)?;

    Ok(())
}
