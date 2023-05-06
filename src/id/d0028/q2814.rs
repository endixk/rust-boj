// BOJ 2814 [BROJ]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

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

const MAX: i64 = 1_000_000_001;
fn factor(pv: &Vec<i64>, pi: usize, f: i64, p: i64, k: i64, dep: usize) -> i64 {
    let mut ret = 0;
    for i in pi..pv.len() {
        if pv[i] >= p { break }
        let nf = f * pv[i];
        if nf > k { break }
        ret += (k / nf) * if dep & 1 == 0 { 1 } else { -1 };
        ret += factor(pv, i + 1, nf, p, k, dep + 1);
    }
    ret
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, p) = (next::<i64>(&mut it), next::<i64>(&mut it));
    let sz = 31777;
    let mut isp = vec![true; sz];
    let mut pv = vec![];
    for i in 2..sz {
        if isp[i] {
            pv.push(i as i64);
            for j in (i..sz).step_by(i) {
                isp[j] = false;
            }
        }
    }

    let (mut lo, mut hi) = (1, MAX);
    while lo < hi {
        let mid = (lo + hi) >> 1;
        if mid / p + factor(&pv, 0, p, p, mid, 1) < n {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    writeln!(so, "{}", if lo == MAX { 0 } else { lo }).ok();
}
