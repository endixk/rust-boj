// BOJ 14413 [Poklon]
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

fn add(cnt: &mut Vec<usize>, v: &mut usize, x: usize) {
    cnt[x] += 1;
    if cnt[x] == 2 { *v += 1; }
    else if cnt[x] == 3 { *v -= 1; }
}
fn del(cnt: &mut Vec<usize>, v: &mut usize, x: usize) {
    cnt[x] -= 1;
    if cnt[x] == 1 { *v -= 1; }
    else if cnt[x] == 2 { *v += 1; }
}
fn mo(ans: &mut Vec<usize>, arr: &[usize], qry: &[Query]) {
    let mut cnt = vec![0; 500001];
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
}

use std::collections::HashMap;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, q) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut arr = vec![];
    let (mut map, mut h) = (HashMap::new(), 0);
    (0..n).for_each(|_| {
        let x = next::<usize>(&mut it);
        if !map.contains_key(&x) { map.insert(x, h); h += 1; }
        arr.push(*map.get(&x).unwrap());
    });

    let mut qry = (0..q).map(|i| {
        Query::new(next::<usize>(&mut it) - 1, next::<usize>(&mut it) - 1, n, i)
    }).collect::<Vec<_>>();
    qry.sort_unstable();
    let mut ans = vec![0; q];
    mo(&mut ans, &arr, &qry);
    ans.iter().for_each(|&x| writeln!(so, "{}", x).unwrap());
}
