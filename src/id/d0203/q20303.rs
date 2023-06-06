// BOJ 20303 [Halloween Bully]
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

fn color(adj: &Vec<Vec<usize>>, cv: &mut Vec<u16>, i: usize, c: u16) {
    cv[i] = c;
    for &j in &adj[i] {
        if cv[j] == 0 {
            color(adj, cv, j, c);
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m, k) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let c = (0..n).map(|_| next::<u32>(&mut it)).collect::<Vec<_>>();

    let mut adj = vec![vec![]; n];
    (0..m).for_each(|_| {
        let (u, v) = (next::<usize>(&mut it) - 1, next::<usize>(&mut it) - 1);
        adj[u].push(v);
        adj[v].push(u);
    });
    let mut co = 1;
    let mut cv = vec![0u16; n];
    for i in 0..n {
        if cv[i] == 0 {
            color(&adj, &mut cv, i, co);
            co += 1;
        }
    }


    let mut knap = vec![(0u32, 0u16); n];
    for i in 0..n {
        let r = cv[i] as usize - 1;
        knap[r].0 += c[i];
        knap[r].1 += 1;
    }
    knap = knap.into_iter().filter(|&(_, cnt)| cnt > 0).collect::<Vec<_>>();

    let mut dp = vec![0; k];
    for i in 1..=knap.len() {
        let mut tp = dp.clone();
        for j in 0..k {
            if j >= knap[i-1].1 as usize {
                tp[j] = dp[j].max(dp[j - knap[i-1].1 as usize] + knap[i-1].0);
            }
        }
        dp = tp;
    }

    println!("{}", dp[k-1]);
}
