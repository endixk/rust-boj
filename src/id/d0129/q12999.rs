// BOJ 12999 [Fancy Village 3]
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

const MAX: usize = 200000;
fn add(cnt: &mut Vec<usize>, ccnt: &mut Vec<usize>, v: &mut usize, x: usize) {
    cnt[x] += 1;
    ccnt[cnt[x] - 1] -= 1;
    ccnt[cnt[x]] += 1;
    if ccnt[*v + 1] > 0 { *v += 1; }
}
fn del(cnt: &mut Vec<usize>, ccnt: &mut Vec<usize>, v: &mut usize, x: usize) {
    cnt[x] -= 1;
    ccnt[cnt[x] + 1] -= 1;
    ccnt[cnt[x]] += 1;
    if ccnt[*v] == 0 { *v -= 1; }
}
fn mo(ans: &mut Vec<usize>, arr: &[usize], qry: &[Query]) {
    let mut cnt = vec![0; MAX+1];
    let mut ccnt = vec![0; MAX+1];
    ccnt[0] = MAX;
    let mut v = 0;
    for i in qry[0].s..=qry[0].e {
        add(&mut cnt, &mut ccnt, &mut v, arr[i]);
    }
    ans[qry[0].i] = v;

    let (mut s, mut e) = (qry[0].s, qry[0].e);
    for &q in qry.iter().skip(1) {
        while q.s < s { add(&mut cnt, &mut ccnt, &mut v, arr[s - 1]); s -= 1; }
        while e < q.e { add(&mut cnt, &mut ccnt, &mut v, arr[e + 1]); e += 1; }
        while s < q.s { del(&mut cnt, &mut ccnt, &mut v, arr[s]); s += 1; }
        while q.e < e { del(&mut cnt, &mut ccnt, &mut v, arr[e]); e -= 1; }
        ans[q.i] = v;
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, q) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let arr = (0..n).map(|_| (next::<i32>(&mut it) + 100000) as usize).collect::<Vec<_>>();
    let mut qry = (0..q).map(|i| {
        Query::new(next::<usize>(&mut it) - 1, next::<usize>(&mut it) - 1, n, i)
    }).collect::<Vec<_>>();
    qry.sort_unstable();

    let mut ans = vec![0; q];
    mo(&mut ans, &arr, &qry);
    for x in ans { writeln!(so, "{}", x).unwrap(); }
}
