// BOJ 15686 [Chicken Delivery]
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

fn dist(p: u16, q: u16, n: usize) -> u16 {
    let n = n as u16;
    let (px, py) = (p / n, p % n);
    let (qx, qy) = (q / n, q % n);
    (px.max(qx) - px.min(qx)) + (py.max(qy) - py.min(qy))
}

fn cdist(pv: &Vec<u16>, qv: &Vec<u16>, n: usize) -> u16 {
    let mut ret = 0;
    for &p in pv {
        let mut d = u16::MAX;
        for &q in qv {
            d = d.min(dist(p, q, n));
        }
        ret += d;
    }
    ret
}

static mut T: u16 = u16::MAX;
fn solve(perm: &mut Vec<usize>, pv: &Vec<u16>, qv: &Vec<u16>,
         n: usize, m: usize, i: usize) {
    if perm.len() == m {
        let mut qsub = Vec::new();
        for i in perm {
            qsub.push(qv[*i]);
        }
        unsafe {
            T = T.min(cdist(pv, &qsub, n));
        }
        return;
    }
    if qv.len() - i < m - perm.len() {
        return;
    }
    for j in i..qv.len() {
        perm.push(j);
        solve(perm, pv, qv, n, m, j+1);
        perm.pop();
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut pv = Vec::new();
    let mut qv = Vec::new();
    for i in 0..(n*n) as u16 {
        let x = next::<u16>(&mut it);
        if x == 1 {
            pv.push(i);
        } else if x == 2 {
            qv.push(i);
        }
    }

    let mut perm = Vec::new();
    solve(&mut perm, &pv, &qv, n, m, 0);
    unsafe {
        writeln!(so, "{}", T).unwrap();
    }
}
