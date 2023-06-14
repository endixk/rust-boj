// BOJ 10538 [Big Picture]
// Supported by GitHub Copilot

use std::io::{self, Read};
use std::collections::{HashMap, VecDeque};

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

struct TrieNode {
    next: Vec<usize>,
    fail: usize,
    end: bool,
}
impl TrieNode {
    fn new(size: usize) -> Self {
        Self {
            next: vec![0; size],
            fail: 0,
            end: false,
        }
    }
}

fn hash(base: char) -> usize {
    match base { 'o' => 0, 'x' => 1, _ => unreachable!() }
}

fn insert(trie: &mut Vec<TrieNode>, s: &str, size: usize) {
    let mut cur = 0;
    for c in s.chars() {
        let i = hash(c);
        if trie[cur].next[i] == 0 {
            trie.push(TrieNode::new(size));
            trie[cur].next[i] = trie.len() - 1;
        }
        cur = trie[cur].next[i];
    }
    trie[cur].end = true;
}

fn build(trie: &mut Vec<TrieNode>, size: usize) {
    let mut q = VecDeque::new();
    q.push_back(0);
    while let Some(cur) = q.pop_front() {
        for i in 0..size {
            let next = trie[cur].next[i];
            if next == 0 { continue; }

            if cur == 0 {
                trie[next].fail = 0;
            } else {
                let mut f = trie[cur].fail;
                while f != 0 && trie[f].next[i] == 0 {
                    f = trie[f].fail;
                }
                if trie[f].next[i] != 0 {
                    f = trie[f].next[i];
                }
                trie[next].fail = f;
            }

            trie[next].end |= trie[trie[next].fail].end;
            q.push_back(next);
        }
    }
}

fn find(trie: &Vec<TrieNode>, s: &str,
        map: &HashMap<String, i32>, x: usize, l: usize,
        locs: &mut Vec<Vec<i32>>) {
    let mut cur = 0;
    for (y, c) in s.chars().enumerate() {
        let i = hash(c);
        while cur != 0 && trie[cur].next[i] == 0 {
            cur = trie[cur].fail;
        }
        if trie[cur].next[i] != 0 {
            cur = trie[cur].next[i];
        }
        if trie[cur].end {
            locs[x][y+1-l] = *map.get(&s[y+1-l..y+1]).unwrap();
        }
    }
}

fn pi(s: &[i32]) -> Vec<usize> {
    let l = s.len();
    let mut p = vec![0; l];

    let mut j = 0;
    for i in 1..l {
        while j > 0 && s[i] != s[j] { j = p[j-1]; }
        if s[i] == s[j] { j += 1; p[i] = j; }
    }

    p
}

fn kmp(s: &[i32], t: &[i32]) -> u32 {
    let p = pi(t);
    let (n, m) = (s.len(), t.len());

    let (mut occ, mut j) = (0, 0);
    for i in 0..n {
        while j > 0 && s[i] != t[j] { j = p[j-1]; }
        if s[i] == t[j] {
            if j == m-1 { occ += 1; j = p[j]; }
            else { j += 1; }
        }
    }
    occ
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (hp, wp, hm, wm) = (
        next::<usize>(&mut it), next::<usize>(&mut it),
        next::<usize>(&mut it), next::<usize>(&mut it));

    let pv = (0..hp).map(|_| next::<String>(&mut it)).collect::<Vec<_>>();
    let mv = (0..hm).map(|_| next::<String>(&mut it)).collect::<Vec<_>>();

    let mut map = HashMap::new();
    let mut x = 0;
    let mut xv = Vec::new();
    for p in pv {
        let val = map.entry(p.clone()).or_insert_with(|| { x += 1; x });
        xv.push(*val);
    }

    let mut trie = vec![TrieNode::new(2)];
    for k in map.keys() {
        insert(&mut trie, k, 2);
    }
    build(&mut trie, 2);

    let mut locs = vec![vec![0; wm]; hm];
    for (x, m) in mv.iter().enumerate() {
        find(&trie, m, &map, x, wp, &mut locs);
    }

    let mut tp = vec![vec![]; wm];
    for v in locs {
        for (i, &val) in v.iter().enumerate() {
            tp[i].push(val);
        }
    }

    let mut ans = 0;
    for v in tp {
        ans += kmp(&v, &xv);
    }

    println!("{}", ans);
}
