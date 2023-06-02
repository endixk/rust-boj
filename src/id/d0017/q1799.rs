// BOJ 1799 [Bishop]
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

fn solve(diag: &Vec<Vec<bool>>, opd: &mut Vec<bool>, k: usize, n: usize) -> usize {
    let mut rd = if k<<1 > n-1 { ((k<<1)-n+1)>>1 } else { (n-1-(k<<1))>>1 };
    if k == n { return 0 }
    let mut ret = 0;
    for &x in &diag[k] {
        if x && !opd[rd] {
            opd[rd] = true;
            ret = ret.max(1 + solve(diag, opd, k+1, n));
            opd[rd] = false;
        }
        rd += 1;
    }
    ret = ret.max(solve(diag, opd, k+1, n));
    ret
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    if n == 1 {
        writeln!(so, "{}", next::<usize>(&mut it) as u8).ok();
        return;
    }

    let mut ev = vec![vec![]; n];
    let mut ov = vec![vec![]; n-1];
    for i in 0..n { for j in 0..n {
        let x = next::<usize>(&mut it) == 1;
        if (i+j)&1 == 0 { ev[(i+j)>>1].push(x); }
        else { ov[(i+j)>>1].push(x); }
    }}

    let mut ans = 0;
    let mut opd = vec![false; n];
    ans += solve(&ev, &mut opd, 0, n);
    let mut opd = vec![false; n];
    ans += solve(&ov, &mut opd, 0,n-1);
    writeln!(so, "{}", ans).unwrap();
}
