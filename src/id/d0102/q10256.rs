// BOJ 10256 [Mutation]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};
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
    match base {
        'A' => 0, 'C' => 1, 'G' => 2, 'T' => 3,
        _ => unreachable!(),
    }
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

fn find(trie: &Vec<TrieNode>, s: &str) -> usize {
    let mut cur = 0;
    let mut cnt = 0;
    for c in s.chars() {
        let i = hash(c);
        while cur != 0 && trie[cur].next[i] == 0 {
            cur = trie[cur].fail;
        }
        if trie[cur].next[i] != 0 {
            cur = trie[cur].next[i];
        }
        if trie[cur].end {
            cnt += 1;
        }
    }
    cnt
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let (_, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
        let (s, t) = (next::<String>(&mut it), next::<String>(&mut it));

        let mut trie = vec![TrieNode::new(4)];
        insert(&mut trie, &t, 4);
        for i in 0..m-1 { for j in i+2..=m {
            let t = t.as_bytes();
            let mut x = String::new();
            (0..i).for_each(|k| x.push(t[k] as char));
            (i..j).rev().for_each(|k| x.push(t[k] as char));
            (j..m).for_each(|k| x.push(t[k] as char));
            insert(&mut trie, &x, 4);
        }}

        build(&mut trie, 4);
        writeln!(so, "{}", find(&trie, &s)).unwrap();
    }
}
