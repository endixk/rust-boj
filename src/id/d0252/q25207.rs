// BOJ 25207 [The Curse of Babel]
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

fn dfs(adj: &Vec<Vec<usize>>, par: &Vec<usize>, val: &Vec<u64>, dep: u64,
       log: &mut Vec<u64>, smap: &mut Vec<usize>, emap: &mut Vec<usize>, cur: usize) {
    smap[cur] = log.len();
    log.push((dep << 40) | val[cur]);
    for &nxt in &adj[cur] {
        if nxt != par[cur] {
            dfs(adj, par, val, dep+1, log, smap, emap, nxt);
        }
    }
    emap[cur] = log.len();
    log.push((dep << 40) | val[cur]);
}

fn manacher(s: &[u64], n: usize) -> Vec<usize> {
    let mut a = vec![0; n];
    let (mut r, mut p) = (0, 0);
    for i in 0..n {
        if i <= r {
            a[i] = a[2 * p - i].min(r - i);
        }
        while i > a[i] && i + a[i] + 1 < n && s[i - a[i] - 1] == s[i + a[i] + 1] {
            a[i] += 1;
        }
        if r < i + a[i] {
            r = i + a[i];
            p = i;
        }
    }
    a
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut val = vec![0; n+1];
    for i in 1..=n {
        val[i] = (next::<i64>(&mut it) + 1_000_000_000) as u64;
    }

    let mut adj = vec![vec![]; n+1];
    let mut par = vec![0; n+1];
    for _ in 1..n {
        let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));
        adj[u].push(v);
        adj[v].push(u);
        par[v] = u;
    }
    for i in 1..=n {
        adj[i].sort_unstable();
    }

    let root = par.iter().enumerate().skip(1).find(|&(_, &p)| p == 0).unwrap().0;
    let mut log = vec![];
    let mut smap = vec![0; n+1];
    let mut emap = vec![0; n+1];
    dfs(&adj, &par, &val, 0, &mut log, &mut smap, &mut emap, root);

    let mut s = vec![0; log.len()<<1|1];
    for i in 0..log.len() {
        s[i<<1|1] = log[i];
    }
    let a = manacher(&s, s.len());

    let mut ans = vec![];
    for i in 1..=n {
        let (l, r) = (smap[i], emap[i]);
        if a[l+r+1] >= r-l {
            ans.push(i);
        }
    }

    writeln!(so, "{}", ans.len())?;
    ans.iter().for_each(|&x| write!(so, "{} ", x).unwrap());

    Ok(())
}
