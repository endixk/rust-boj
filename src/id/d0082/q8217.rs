// BOJ 8217 [Meteors]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

struct FenwickTree {
    n: usize,
    ft: Vec<i64>,
}
impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            ft: vec![0; n+1],
        }
    }
    fn sum(&self, mut i: usize) -> i64 {
        let mut sum = 0;
        while i > 0 {
            sum += self.ft[i];
            i -= i & (!i + 1);
        }
        sum
    }
    fn add(&mut self, mut i: usize, v: i64) {
        while i <= self.n {
            self.ft[i as usize] += v;
            i += i & (!i + 1);
        }
    }
    fn update(&mut self, l: usize, r: usize, v: i64) {
        self.add(l, v);
        self.add(r+1, -v);
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it)); // n: states, m: sectors

    let own = (0..=m).map(|i| if i == 0 { 0 } else { next::<usize>(&mut it) }).collect::<Vec<_>>(); // sector owner
    let mut sec = vec![vec![]; n+1]; // sectors per state
    for (i, &o) in own.iter().enumerate() {
        sec[o].push(i);
    }
    let quo = (0..=n).map(|i| if i == 0 { 0 } else { next::<i64>(&mut it) }).collect::<Vec<_>>(); // quotas

    let q = next::<usize>(&mut it);
    let mut qs = vec![(0, 0, 0)];
    (0..q).for_each(|_| {
        qs.push((next::<usize>(&mut it), next::<usize>(&mut it), next::<i64>(&mut it)));
    });

    // parallel binary search
    let mut lo = vec![0usize; n+1];
    let mut hi = vec![q+1; n+1];
    loop {
        // check if all states fulfilled their quota
        let mut idx = vec![vec![]; q+1];
        let mut cnt = 0;
        lo.iter().zip(hi.iter()).enumerate().for_each(|(i, (&l, &h))| {
            if l+1 < h {
                idx[(l + h) >> 1].push(i);
                cnt += 1;
            }
        });
        if cnt == 0 { break; }

        // process queries
        let mut ft = FenwickTree::new(m+1);
        for (i, &(l, r, a)) in qs.iter().enumerate() {
            if i == 0 { continue; }
            if l <= r {
                ft.update(l, r, a);
            } else {
                ft.update(1, r, a);
                ft.update(l, m, a);
            }

            for &j in idx[i].iter() {
                let mut sum = 0;
                for &k in sec[j].iter() {
                    sum += ft.sum(k);
                    if sum >= quo[j] { break; }
                }
                if sum >= quo[j] {
                    hi[j] = (lo[j] + hi[j]) >> 1;
                } else {
                    lo[j] = (lo[j] + hi[j]) >> 1;
                }
            }
        }
    }

    // output
    for i in 1..=n {
        writeln!(so, "{}", if hi[i] == q+1 { "NIE".to_string() } else { hi[i].to_string() }).ok();
    }
}
