// BOJ 1253 [Good]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut v = (0..next(&mut it)).map(|_| next(&mut it)).collect::<Vec<i32>>();
    v.sort();

    let mut r = Vec::new();
    for i in 0..v.len()-1 {
        for j in i+1..v.len() {
            let sum = v[i] + v[j];
            let lb = v.partition_point(|&x| x < sum);
            let ub = v.partition_point(|&x| x <= sum);
            let req = (sum == v[i]) as usize + (sum == v[j]) as usize;
            if ub - lb > req {
                r.push(sum);
            }
        }
    }
    r.sort();

    let (mut vi, mut ri, mut ans) = (0, 0, 0);
    while vi < v.len() && ri < r.len() {
        if v[vi] < r[ri] {
            vi += 1;
        } else if v[vi] > r[ri] {
            ri += 1;
        } else {
            let (vc, rc) = (vi, ri);
            while vi < v.len() && v[vi] == v[vc] { vi += 1; }
            while ri < r.len() && r[ri] == r[rc] { ri += 1; }
            ans += vi - vc;
        }
    }

    writeln!(so, "{}", ans).unwrap();
}
