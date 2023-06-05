// BOJ 8992 [Picking Game]
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
const SNK: usize = 401;
const MAX: usize = 402;
const INF: i32 = 0x3f3f3f3f;
fn mcmf(adj: &Vec<Vec<usize>>, cst: &Vec<Vec<i32>>,
        flo: &mut Vec<Vec<i32>>, cap: &mut Vec<Vec<i32>>) -> (i32, i32) {
    let mut csum = 0;
    let mut fsum = 0;
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
        fsum += mflo;
    }
    (fsum, csum)
}

fn isct(hor: (i32, i32, i32), ver: (i32, i32, i32)) -> bool {
    // hor: (x1, x2, y), ver: (y1, y2, x)
    hor.0 < ver.2 && ver.2 < hor.1 && ver.0 < hor.2 && hor.2 < ver.1
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let mut adj = vec![vec![]; MAX];
        let mut cst = vec![vec![0; MAX]; MAX];
        let mut flo = vec![vec![0; MAX]; MAX];
        let mut cap = vec![vec![0; MAX]; MAX];

        let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
        let (mut hors, mut vers) = (vec![], vec![]);
        for i in 1..=n {
            let (x1, y, x2, _, w) = (
                next::<i32>(&mut it), next::<i32>(&mut it),
                next::<i32>(&mut it), next::<i32>(&mut it),
                next::<i32>(&mut it));
            if x1 > x2 { hors.push((x2, x1, y, w)); }
            else { hors.push((x1, x2, y, w)); }

            adj[SRC].push(i);
            adj[i].push(SRC);
            cap[SRC][i] = 1;
        }
        for j in n+1..=n+m {
            let (x, y1, _, y2, w) = (
                next::<i32>(&mut it), next::<i32>(&mut it),
                next::<i32>(&mut it), next::<i32>(&mut it),
                next::<i32>(&mut it));
            if y1 > y2 { vers.push((y2, y1, x, w)); }
            else { vers.push((y1, y2, x, w)); }

            adj[j].push(SNK);
            adj[SNK].push(j);
            cap[j][SNK] = 1;
        }
        for i in 1..=n { for j in n+1..=n+m {
            if isct((hors[i-1].0, hors[i-1].1, hors[i-1].2), (vers[j-n-1].0, vers[j-n-1].1, vers[j-n-1].2)) {
                adj[i].push(j);
                adj[j].push(i);
                cst[i][j] = -hors[i-1].3 * vers[j-n-1].3;
                cst[j][i] = hors[i-1].3 * vers[j-n-1].3;
                cap[i][j] = 1;
            }
        }}

        let (fsum, csum) = mcmf(&adj, &cst, &mut flo, &mut cap);
        writeln!(so, "{} {}", fsum, -csum).unwrap();
    }
}
