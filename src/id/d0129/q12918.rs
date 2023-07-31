// BOJ 12918 [Cleaning Up]
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

use std::collections::VecDeque;
const SRC: usize = 0;
const RSK: usize = 201;
const SNK: usize = 202;
const MAX: usize = 203;
const INF: f64 = 1e9;
fn mcmf(adj: &Vec<Vec<usize>>, cst: &Vec<Vec<f64>>,
        flo: &mut Vec<Vec<i32>>, cap: &mut Vec<Vec<i32>>) -> f64 {
    let mut csum = 0.0;
    loop {
        let mut prv = vec![MAX; MAX];
        let mut dst = vec![INF; MAX];
        dst[SRC] = 0.0;
        prv[SRC] = SRC;

        let mut q = VecDeque::new();
        let mut inq = vec![false; MAX];
        inq[SRC] = true;
        q.push_back(SRC);

        // SPFA
        while let Some(cur) = q.pop_front() {
            inq[cur] = false;
            for &nxt in &adj[cur] {
                if cap[cur][nxt] > flo[cur][nxt] && dst[nxt] > dst[cur] + cst[cur][nxt] {
                    dst[nxt] = dst[cur] + cst[cur][nxt];
                    prv[nxt] = cur;
                    if !inq[nxt] {
                        inq[nxt] = true;
                        q.push_back(nxt);
                    }
                }
            }
        }
        if prv[SNK] == MAX { break }

        // find min flow
        let mut mflo = 0x3f3f3f3f;
        let mut cur = SNK;
        while cur != SRC {
            let p = prv[cur];
            mflo = mflo.min(cap[p][cur] - flo[p][cur]);
            cur = p;
        }

        // update flow
        let mut cur = SNK;
        while cur != SRC {
            let p = prv[cur];
            csum += mflo as f64 * cst[p][cur];
            flo[p][cur] += mflo;
            flo[cur][p] -= mflo;
            cur = p;
        }
    }
    csum
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let (mut l, mut r) = (vec![], vec![]);
    for _ in 0..n {
        let (x, y) = (next::<f64>(&mut it), next::<f64>(&mut it));
        if x < 1e-9 { l.push((x, y)) }
        else if x > 1e-9 { r.push((x, y)) }
    }

    let mut adj = vec![vec![]; MAX];
    let mut cst = vec![vec![0.0; MAX]; MAX];
    let mut flo = vec![vec![0; MAX]; MAX];
    let mut cap = vec![vec![0; MAX]; MAX];
    // connect source to left
    for i in 0..l.len() {
        adj[SRC].push(i+1);
        adj[i+1].push(SRC);
        cap[SRC][i+1] = 1;
    }
    // connect right to sink
    for i in 0..r.len() {
        adj[i+101].push(SNK);
        adj[SNK].push(i+101);
        cap[i+101][SNK] = 1;
    }
    // connect left to right
    for i in 0..l.len() {
        for j in 0..r.len() {
            let d = (l[i].0 + r[j].0).hypot(l[i].1 - r[j].1);
            adj[i+1].push(j+101);
            adj[j+101].push(i+1);
            cst[i+1][j+101] = d - r[j].0;
            cst[j+101][i+1] = -d + r[j].0;
            cap[i+1][j+101] = 1;
        }
    }
    // connect right sink
    adj[RSK].push(SNK);
    adj[SNK].push(RSK);
    cap[RSK][SNK] = l.len() as i32;
    for i in 0..l.len() {
        adj[i+1].push(RSK);
        adj[RSK].push(i+1);
        cst[i+1][RSK] = -l[i].0;
        cst[RSK][i+1] = l[i].0;
        cap[i+1][RSK] = 1;
    }

    // mcmf
    let cost = mcmf(&adj, &cst, &mut flo, &mut cap);
    println!("{:.3}", cost + r.iter().map(|&(x, _)| x).sum::<f64>());
}
