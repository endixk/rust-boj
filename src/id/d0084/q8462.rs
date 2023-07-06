// BOJ 8462 [Power of Array]
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Query {
    s: usize,
    e: usize,
    k: usize,
    i: usize,
}
impl Query {
    fn new(s: usize, e: usize, n: usize, i: usize) -> Self {
        Self { s, e, k: (n as f64).sqrt() as usize, i }
    }
}
impl Ord for Query {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.s / self.k).cmp(&(other.s / other.k))
            .then(self.e.cmp(&other.e))
    }
}
impl PartialOrd for Query {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn add(cnt: &mut Vec<u32>, v: &mut usize, x: usize) {
    cnt[x] += 1;
    *v += (cnt[x] * cnt[x] - (cnt[x] - 1) * (cnt[x] - 1)) as usize * x;
}
fn del(cnt: &mut Vec<u32>, v: &mut usize, x: usize) {
    cnt[x] -= 1;
    *v -= ((cnt[x] + 1) * (cnt[x] + 1) - cnt[x] * cnt[x]) as usize * x;
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, t) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let arr = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let mut qry = (0..t).map(|i| {
        Query::new(next::<usize>(&mut it) - 1, next::<usize>(&mut it) - 1, n, i)
    }).collect::<Vec<_>>();
    qry.sort_unstable();

    let mut cnt = vec![0; 1000001];
    let mut ans = vec![0; t];
    let mut v = 0;
    for i in qry[0].s..=qry[0].e {
        add(&mut cnt, &mut v, arr[i]);
    }
    ans[qry[0].i] = v;

    let (mut s, mut e) = (qry[0].s, qry[0].e);
    for &q in qry.iter().skip(1) {
        while q.s < s { add(&mut cnt, &mut v, arr[s - 1]); s -= 1; }
        while e < q.e { add(&mut cnt, &mut v, arr[e + 1]); e += 1; }
        while s < q.s { del(&mut cnt, &mut v, arr[s]); s += 1; }
        while q.e < e { del(&mut cnt, &mut v, arr[e]); e -= 1; }
        ans[q.i] = v;
    }

    for a in ans { writeln!(so, "{}", a).unwrap(); }
}
