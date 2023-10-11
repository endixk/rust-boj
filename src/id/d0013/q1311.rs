// BOJ 1311 [Assignment 1]
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

use std::collections::VecDeque;
const SRC: usize = 0;
const SNK: usize = 41;
const MAX: usize = 42;
const INF: i32 = 0x3f3f3f3f;
fn mcmf(adj: &Vec<Vec<usize>>, cst: &Vec<Vec<i32>>,
        flo: &mut Vec<Vec<i32>>, cap: &mut Vec<Vec<i32>>) -> i32 {
    let mut csum = 0;
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
    }
    csum
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut adj = vec![vec![]; MAX];
    let mut cst = vec![vec![0; MAX]; MAX];
    let mut flo = vec![vec![0; MAX]; MAX];
    let mut cap = vec![vec![0; MAX]; MAX];

    for i in 1..=n {
        adj[SRC].push(i);
        adj[i].push(SRC);
        cap[SRC][i] = 1;

        adj[i + 20].push(SNK);
        adj[SNK].push(i + 20);
        cap[i + 20][SNK] = 1;

        for j in 1..=n {
            cst[i][j + 20] = next::<i32>(&mut it);
            cst[j + 20][i] = -cst[i][j + 20];
            adj[i].push(j + 20);
            adj[j + 20].push(i);
            cap[i][j + 20] = 1;
        }
    }

    writeln!(so, "{}", mcmf(&adj, &cst, &mut flo, &mut cap))?;

    Ok(())
}
