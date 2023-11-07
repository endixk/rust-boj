// BOJ 30410 [Dish Overlapping]
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
    let a = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();

    let (mut c, mut k) = (a[0], 1);
    let mut v = vec![];
    for i in 1..n {
        if a[i] == c { k += 1; }
        else { v.push((c, k)); c = a[i]; k = 1; }
    }
    v.push((c, k));

    let mut i = 0;
    let mut w = vec![(2, 0)];
    for (c, k) in v {
        if c == 2 {
            if w.last().unwrap().0 == 2 {
                w[i].1 += k;
            } else {
                w.push((2, k));
                i += 1;
            }
        } else if k % 2 == 0 {
            if w.last().unwrap().0 == 2 {
                w[i].1 += k>>1;
            } else {
                w.push((2, k>>1));
                i += 1;
            }
        } else {
            w.push((1, k));
            i += 1;
        }
    }
    w.push((2, 0));

    let mut ans = 1;
    for i in 0..w.len() {
        if w[i].0 == 2 {
            let mut k = w[i].1 << 1;
            if i > 1 { k += w[i-1].1 >> 1 << 1; }
            if i+1 < w.len() { k += w[i+1].1 >> 1 << 1; }

            let mut z = 1;
            while z <= k { z <<= 1; }
            ans = ans.max(z >> 1);
        }
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
