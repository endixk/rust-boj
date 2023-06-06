// BOJ 16115 [Fan]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::f64::consts::PI;

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
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut v = Vec::with_capacity(n);

    let mut dmax = 0;
    (0..n).for_each(|_| {
        let (x, y) = (next::<i64>(&mut it), next::<i64>(&mut it));
        let d = x * x + y * y;
        v.push((x, y, d));
        if d > dmax { dmax = d; }
    });

    let v = v.into_iter().filter(|(_, _, d)| *d == dmax)
        .map(|(x, y, _)| (x as f64, y as f64))
        .collect::<Vec<_>>();
    let (mut s, mut t) = (0, PI);
    for (i, &(x, y)) in v.iter().enumerate() {
        let t2 = y.atan2(x);
        if t2 < t { t = t2; s = i; }
    }

    let (mut tmax, mut tsum) = (0.0, 0.0);
    for i in 0..v.len()-1 {
        let (i, j) = ((s + i) % v.len(), (s + i + 1) % v.len());
        let dt = (v[j].1).atan2(v[j].0) - (v[i].1).atan2(v[i].0);
        if dt > tmax { tmax = dt; }
        tsum += dt;
    }
    tmax = tmax.max(2.0 * PI - tsum);
    println!("{:.6}", tmax * 180.0 / PI);
}
