// BOJ 3648 [Idol]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};

fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::iter::Peekable<std::str::SplitAsciiWhitespace>) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

fn tarjan(
    scc: &mut Vec<Vec<usize>>,
    adj: &Vec<Vec<usize>>,
    ids: &mut Vec<usize>,
    vis: &mut Vec<bool>,
    stk: &mut Vec<usize>,
    cnt: &mut usize,
    cur: usize) -> usize {

    *cnt += 1;
    let mut ret = *cnt;
    vis[cur] = true;
    ids[cur] = *cnt;
    stk.push(cur);

    for &nxt in &adj[cur] {
        if ids[nxt] == 0 {
            ret = ret.min(tarjan(scc, adj, ids, vis, stk, cnt, nxt));
        } else if vis[nxt] {
            ret = ret.min(ids[nxt]);
        }
    }

    if ret == ids[cur] {
        let mut scc_cur = Vec::new();
        loop {
            let top = stk.pop().unwrap();
            scc_cur.push(top);
            vis[top] = false;
            if top == cur { break; }
        }
        scc_cur.sort_unstable();
        scc.push(scc_cur);
    }

    ret
}

fn node(x: i32, n: usize) -> usize {
    if x > 0 {
        x as usize
    } else {
        -x as usize + n
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace().peekable();

    'tc: while let Some(_) = it.peek() {
        let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
        let size = (n<<1)+1;
        let mut adj: Vec<Vec<usize>> = vec![vec![]; size];
        for _ in 0..m {
            let (x, y) = (next::<i32>(&mut it), next::<i32>(&mut it));
            adj[node(-x, n)].push(node(y, n));
            adj[node(-y, n)].push(node(x, n));
        }
        adj[node(-1, n)].push(node(1, n));

        let mut scc: Vec<Vec<usize>> = Vec::new();
        let mut ids: Vec<usize> = vec![0; size];
        let mut vis: Vec<bool> = vec![false; size];
        let mut stk: Vec<usize> = Vec::new();
        let mut cnt = 0;
        for i in 1..size {
            if ids[i] == 0 {
                tarjan(&mut scc, &adj, &mut ids, &mut vis, &mut stk, &mut cnt, i);
            }
        }

        for cc in scc {
            for &v in cc.iter() {
                if v > n { continue; }
                if cc.binary_search(&(v+n)).is_ok() {
                    writeln!(so, "no").unwrap();
                    continue 'tc;
                }
            }
        }
        writeln!(so, "yes").unwrap();
    }
}
