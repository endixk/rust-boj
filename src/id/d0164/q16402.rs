// BOJ 16402 [Empires]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn read<T>(si: &mut T) -> String where T: BufRead {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn find(root: &mut Vec<usize>, x: usize) -> usize {
    if root[x] == x {
        x
    } else {
        root[x] = find(root, root[x]);
        root[x]
    }
}

fn union(root: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
    let x = find(root, x);
    let y = find(root, y);
    if x == y {
        return;
    }
    if rank[x] < rank[y] {
        root[x] = y;
    } else {
        root[y] = x;
        if rank[x] == rank[y] {
            rank[x] += 1;
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);

    let v = s.split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let (n, m) = (v[0], v[1]);
    let mut evec = Vec::new();
    let mut emap = HashMap::new();
    for i in 0..n {
        let e = read(&mut si);
        evec.push(e.clone());
        emap.insert(e, i);
    }

    let mut root = (0..n).collect::<Vec<usize>>();
    let mut rank = vec![0; n];
    let mut lead = (0..n).collect::<Vec<usize>>();
    for _ in 0..m {
        let v = read(&mut si).split(',').map(|x| x.to_string()).collect::<Vec<String>>();
        let (k1, k2, w) = (emap[&v[0]], emap[&v[1]], v[2].parse::<usize>().unwrap());
        let (l1, l2) = (lead[find(&mut root, k1)], lead[find(&mut root, k2)]);
        if find(&mut root, k1) != find(&mut root, k2) {
            union(&mut root, &mut rank, k1, k2);
            if w == 1 { lead[find(&mut root, k1)] = l1; }
            if w == 2 { lead[find(&mut root, k2)] = l2; }
        } else {
            if w == 1 { lead[find(&mut root, k1)] = k1; }
            else { lead[find(&mut root, k2)] = k2; }
        }

    }

    let mut leads = Vec::new();
    for i in 0..n {
        if find(&mut root, i) == i {
            leads.push(evec[lead[i]].clone());
        }
    }
    leads.sort();

    writeln!(so, "{}", leads.len()).ok();
    leads.into_iter().for_each(|x| writeln!(so, "{}", x).unwrap());
}
