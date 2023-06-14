// BOJ 2809 [POPLOČAVANJE]
// Supported by GitHub Copilot

use std::io::{self, BufRead};
use std::collections::VecDeque;

fn read<T>(si: &mut T) -> String where T: BufRead {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

struct TrieNode {
    next: Vec<usize>,
    fail: usize,
    end: usize,
}
impl TrieNode {
    fn new(size: usize) -> Self {
        Self {
            next: vec![0; size],
            fail: 0,
            end: 0,
        }
    }
}

fn hash(base: char) -> usize {
    (base as usize) - ('a' as usize)
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
    trie[cur].end = s.len();
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

            trie[next].end = trie[next].end.max(trie[trie[next].fail].end);
            q.push_back(next);
        }
    }
}

fn find(trie: &Vec<TrieNode>, s: &str, imos: &mut Vec<i32>) {
    let mut cur = 0;
    for (k, c) in s.chars().enumerate() {
        let i = hash(c);
        while cur != 0 && trie[cur].next[i] == 0 {
            cur = trie[cur].fail;
        }
        if trie[cur].next[i] != 0 {
            cur = trie[cur].next[i];
        }
        if trie[cur].end > 0 {
            imos[k+1-trie[cur].end] += 1;
            imos[k+1] -= 1;
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());

    let n = read(&mut si).parse::<usize>().unwrap();
    let s = read(&mut si);
    let m = read(&mut si).parse::<usize>().unwrap();

    let mut imos = vec![0; n+1];
    let mut i = 0;
    for _ in 0..=m/500 {
        let mut trie = vec![TrieNode::new(26)];
        for _ in 0..500 {
            let t = read(&mut si);
            insert(&mut trie, &t, 26);
            i += 1;
            if i == m { break; }
        }
        build(&mut trie, 26);
        find(&trie, &s, &mut imos);
        if i == m { break; }
    }

    let (mut ans, mut cnt) = (0, 0);
    for i in 0..n {
        cnt += imos[i];
        if cnt == 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
