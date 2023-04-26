// BOJ 1396 [Kruskal's Ball]
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

fn find(root: &mut Vec<usize>, x: usize) -> usize {
    if root[x] == x {
        x
    } else {
        root[x] = find(root, root[x]);
        root[x]
    }
}
fn union_size(root: &mut Vec<usize>,
              rank: &mut Vec<usize>,
              size: &mut Vec<usize>, x: usize, y: usize) -> usize {
    let x = find(root, x);
    let y = find(root, y);
    if x != y {
        if rank[x] < rank[y] {
            root[x] = y;
            size[y] += size[x];
            size[y]
        } else {
            root[y] = x;
            size[x] += size[y];
            if rank[x] == rank[y] {
                rank[x] += 1;
            }
            size[x]
        }
    } else {
        size[x]
    }
}

fn pbs_kruskal(edges: &Vec<(usize, usize, usize)>,
               queries: &Vec<(usize, usize)>, n: usize, m: usize, q: usize) -> Vec<Option<(usize, usize)>> {
    let mut lo = vec![0 as usize; q];
    let mut hi = vec![m+1; q];
    let mut sz = vec![0 as usize; q];

    // parallel binary search
    loop {
        let mut qidx = vec![Vec::<usize>::with_capacity(q); m+1];
        let mut cnt = 0;
        lo.iter().zip(hi.iter()).enumerate().for_each(|(i, (l, h))| {
            if *l+1 < *h {
                qidx[(*l+*h)>>1].push(i);
                cnt += 1;
            }
        });
        if cnt == 0 { break }

        // run Kruskal's algorithm
        let mut root = (0..n).collect::<Vec<usize>>();
        let mut rank = vec![0 as usize; n];
        let mut size = vec![1 as usize; n];
        for (x, (_, u, v)) in edges.iter().enumerate() {
            if find(&mut root, *u) != find(&mut root, *v) {
                union_size(&mut root, &mut rank, &mut size, *u, *v);
            }
            for i in qidx[x+1].iter() {
                let (qu, qv) = queries[*i];
                if find(&mut root, qu) == find(&mut root, qv) {
                    sz[*i] = size[find(&mut root, qu)];
                    hi[*i] = (lo[*i]+hi[*i])>>1;
                } else {
                    lo[*i] = (lo[*i]+hi[*i])>>1;
                }
            }
        }
    }

    let mut ret = Vec::new();
    hi.iter().zip(sz.iter()).for_each(|(h, s)| {
        ret.push(if *s == 0 { None } else { Some((edges[*h-1].0, *s)) });
    });
    ret
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));

    let mut edges = Vec::new();
    for _ in 0..m {
        let (u, v, w) = (next::<usize>(&mut it)-1, next::<usize>(&mut it)-1, next::<usize>(&mut it));
        edges.push((w, u, v));
    }
    edges.sort();

    let q = next::<usize>(&mut it);
    let mut queries = Vec::with_capacity(q);
    for _  in 0..q {
        let (u, v) = (next::<usize>(&mut it)-1, next::<usize>(&mut it)-1);
        queries.push((u, v));
    }

    let ans = pbs_kruskal(&edges, &queries, n, m, q);
    ans.iter().for_each(|x| {
        match x {
            Some((w, s)) => writeln!(so, "{} {}", w, s).unwrap(),
            None => writeln!(so, "-1").unwrap(),
        }
    });
}
