// BOJ 6171 [Land Acquisition]
// Supported by GitHub Copilot

use std::io::{self, Read};
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

#[derive(Copy, Clone)]
struct Segment {
    a: i64,
    b: i64,
    s: f64,
}
fn sect(p: &Segment, q: &Segment) -> f64 {
    (q.b - p.b) as f64 / (p.a - q.a) as f64
}
fn find(cht: &[Segment], x: f64) -> usize {
    if x > cht.last().unwrap().s { return cht.len() - 1; }
    let mut lo = 0;
    let mut hi = cht.len() - 1;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if cht[mid].s < x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo - 1
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut v = (0..n).map(|_| {
        (next::<i64>(&mut it), next::<i64>(&mut it))
    }).collect::<Vec<_>>();
    v.sort_unstable();

    let mut a = Vec::new();
    a.push(*v.last().unwrap());
    let mut hmax = a[0].1;
    for &(w, h) in v.iter().rev().skip(1) {
        if h > hmax {
            a.push((w, h));
            hmax = h;
        }
    }
    a.reverse();

    let mut dp = vec![0; a.len()];
    dp[0] = a[0].0 * a[0].1;
    let mut cht = Vec::new();
    cht.push(Segment { a: a[0].1, b: 0, s: 0.0 });
    for i in 1..a.len() {
        // insert new segment
        let mut nseg = Segment { a: a[i].1, b: dp[i - 1], s: 0.0 };
        while let Some(&p) = cht.last() {
            nseg.s = sect(&p, &nseg);
            if p.s < nseg.s {
                break;
            }
            cht.pop();
        }
        cht.push(nseg);

        // find the best segment
        let loc = find(&cht, a[i].0 as f64);
        dp[i] = cht[loc].a * a[i].0 + cht[loc].b;
    }
    println!("{}", dp.last().unwrap());
}
