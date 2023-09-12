// BOJ 2571 [Colored Papers 3]
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

    let mut b = [[false; 110]; 110];
    for _ in 0..next(&mut it) {
        let (p, q) = (next::<usize>(&mut it), next::<usize>(&mut it));
        for i in p+1..p+11 {
            for j in q+1..q+11 {
                b[i][j] = true;
            }
        }
    }

    let mut bits = [0i128; 110];
    for i in 0..110 {
        for j in 0..110 {
            if b[i][j] {
                bits[i] |= 1 << j;
            }
        }
    }

    let mut ans = 0;
    for i in 0..109 {
        for j in i+1..110 {
            let mut cmp = 0;
            for x in i..j { cmp |= 1 << x; }
            let (mut c, mut v) = (0, 0);
            for k in 0..110 {
                if bits[k] & cmp == cmp {
                    c += 1;
                    v = v.max(c);
                } else { c = 0; }
            }
            ans = ans.max(v * (j - i));
        }
    }
    writeln!(so, "{}", ans).unwrap();
}
