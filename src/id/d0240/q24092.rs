// BOJ 24092 [Quick Sort 3]
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

    let n = next::<usize>(&mut it);
    let mut a = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();
    let b = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();
    let mut d = (0..n).fold(0, |acc, i| acc + (a[i] - b[i]).abs());
    if d == 0 {
        writeln!(so, "1").ok();
        return;
    }

    let mut st = Vec::new();
    st.push((0, n-1));
    while let Some((p, r)) = st.pop() {
        if p < r {
            let (x, mut i) = (a[r], p);
            for j in p..r {
                if a[j] <= x {
                    d += (a[i] - b[j]).abs() + (a[j] - b[i]).abs() - (a[i] - b[i]).abs() - (a[j] - b[j]).abs();
                    if d == 0 {
                        writeln!(so, "1").ok();
                        return;
                    }
                    a.swap(i, j);
                    i += 1;
                }
            }
            if i < r {
                d += (a[i] - b[r]).abs() + (a[r] - b[i]).abs() - (a[i] - b[i]).abs() - (a[r] - b[r]).abs();
                if d == 0 {
                    writeln!(so, "1").ok();
                    return;
                }
                a.swap(i, r);
            }
            st.push((i + 1, r));
            if i > 0 { st.push((p, i - 1)); }
        }
    }
    writeln!(so, "0").ok();
}
