// BOJ 11111 [Tofu Seller]
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
const SNK: usize = 2501;
const MAX: usize = 2502;
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

const COST: [[i32; 6]; 6] = [
    [10, 8, 7, 5, 0, 1],
    [8, 6, 4, 3, 0, 1],
    [7, 4, 3, 2, 0, 1],
    [5, 3, 2, 2, 0, 1],
    [0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 0, 0],
];
#[inline] fn cost(a: char, b: char) -> i32 {
    COST[a as usize - 'A' as usize][b as usize - 'A' as usize]
}
#[inline] fn connect(adj: &mut Vec<Vec<usize>>, cst: &mut Vec<Vec<i32>>, cap: &mut Vec<Vec<i32>>,
           x: usize, y: usize, a: char, b: char) {
    adj[x].push(y);
    adj[y].push(x);
    cst[x][y] = -cost(a, b);
    cst[y][x] = -cst[x][y];
    cap[x][y] = 1;
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut b = vec![vec![]; n];
    for i in 0..n {
        for (_, c) in next::<String>(&mut it).chars().enumerate() {
            b[i].push(c);
        }
    }

    let mut adj = vec![vec![]; MAX];
    let mut cst = vec![vec![0; MAX]; MAX];
    let mut flo = vec![vec![0; MAX]; MAX];
    let mut cap = vec![vec![0; MAX]; MAX];

    let mut flag = true;
    for i in 0..n {
        let mut t = flag;
        for j in 0..m {
            let x = i * m + j + 1;
            if t {
                adj[SRC].push(x);
                adj[x].push(SRC);
                cap[SRC][x] = 1;
                if i > 0 { connect(&mut adj, &mut cst, &mut cap, x, x - m, b[i][j], b[i - 1][j]); }
                if j > 0 { connect(&mut adj, &mut cst, &mut cap, x, x - 1, b[i][j], b[i][j - 1]); }
                if i < n - 1 { connect(&mut adj, &mut cst, &mut cap, x, x + m, b[i][j], b[i + 1][j]); }
                if j < m - 1 { connect(&mut adj, &mut cst, &mut cap, x, x + 1, b[i][j], b[i][j + 1]); }
                adj[x].push(SNK);
                adj[SNK].push(x);
                cap[x][SNK] = 1;
            } else {
                adj[x].push(SNK);
                adj[SNK].push(x);
                cap[x][SNK] = 1;
            }
            t = !t;
        }
        flag = !flag;
    }

    writeln!(so, "{}", -mcmf(&adj, &cst, &mut flo, &mut cap))?;

    Ok(())
}
