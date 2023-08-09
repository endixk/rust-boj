// BOJ 28140 [Cotton Candies]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, q) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let a = next::<String>(&mut it);
    let a = a.as_bytes();

    let mut rc = [0; 1000001]; rc[n] = n;
    let mut bc = [0; 1000001]; bc[n] = n;
    let (mut ri, mut bi) = (n, n);
    for i in (0..n).rev() {
        rc[i] = ri;
        bc[i] = bi;
        if a[i] == b'R' { ri = i; }
        else if a[i] == b'B' { bi = i; }
    }

    for _ in 0..q {
        let (l, r) = (next::<usize>(&mut it), next::<usize>(&mut it));
        let (r1, r2) = if a[l] == b'R' { (l, rc[l] ) } else { (rc[l], rc[rc[l]])};
        let (b1, b2) = (bc[r2], bc[bc[r2]]);

        if b2 <= r {
            writeln!(so, "{} {} {} {}", r1, r2, b1, b2).ok();
        } else {
            writeln!(so, "-1").ok();
        }
    }
}
