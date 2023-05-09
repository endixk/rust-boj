// BOJ 11408 [Passionate Kang-ho 5]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

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

const SRC: usize = 0;
const SNK: usize = 801;
const MAX: usize = 802;
const INF: i32 = 0x3f3f3f3f;
fn mcmf(adj: &Vec<Vec<usize>>, cst: &Vec<Vec<i32>>,
        flo: &mut Vec<Vec<i32>>, cap: &mut Vec<Vec<i32>>) -> (i32, i32) {
    let (mut csum, mut cnt) = (0, 0);
    loop {
        let mut prv = vec![MAX; MAX];
        let mut dst = vec![INF; MAX];
        dst[SRC] = 0;
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
        let mut mflo = INF;
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
            csum += mflo * cst[p][cur];
            flo[p][cur] += mflo;
            flo[cur][p] -= mflo;
            cur = p;
        }
        cnt += 1;
    }
    (csum, cnt)
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut adj = vec![vec![]; MAX];
    let mut cst = vec![vec![0; MAX]; MAX];
    let mut flo = vec![vec![0; MAX]; MAX];
    let mut cap = vec![vec![0; MAX]; MAX];

    // make graph
    for i in 1..=n {
        for _ in 0..next(&mut it) {
            let (j, c) = (next::<usize>(&mut it), next::<i32>(&mut it));
            adj[i].push(n + j);
            adj[n + j].push(i);
            cst[i][n + j] = c;
            cst[n + j][i] = -c;
            cap[i][n + j] = 1;
        }
    }
    // connect source
    for i in 1..=n {
        adj[SRC].push(i);
        adj[i].push(SRC);
        cap[SRC][i] = 1;
    }
    // connect sink
    for j in 1..=m {
        adj[n + j].push(SNK);
        adj[SNK].push(n + j);
        cap[n + j][SNK] = 1;
    }

    // solve
    let (csum, cnt) = mcmf(&adj, &cst, &mut flo, &mut cap);
    writeln!(so, "{} {}", cnt, csum).ok();
}
