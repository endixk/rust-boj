// BOJ 24091 [Quick Sort 2]
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

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    let mut c = 0;

    let mut st = Vec::new();
    st.push((0, n-1));
    while let Some((p, r)) = st.pop() {
        if p < r {
            let (x, mut i) = (a[r], p);
            for j in p..r {
                if a[j] <= x {
                    a.swap(i, j);
                    i += 1;
                    c += 1;
                    if c == k {
                        a.iter().for_each(|&x| write!(so, "{} ", x).unwrap());
                        writeln!(so, "").ok();
                        return;
                    }
                }
            }
            if i < r {
                a.swap(i, r);
                c += 1;
                if c == k {
                    a.iter().for_each(|&x| write!(so, "{} ", x).unwrap());
                    writeln!(so, "").ok();
                    return;
                }
            }
            st.push((i + 1, r));
            if i > 0 { st.push((p, i - 1)); }
        }
    }
    writeln!(so, "-1").ok();
}
