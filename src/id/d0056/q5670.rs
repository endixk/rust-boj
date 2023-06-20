// BOJ 5670 [Cellphone Typing]
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

struct TrieNode {
    next: [usize; 26],
    branch: usize,
    cnt: usize,
}
impl TrieNode {
    fn new() -> Self {
        Self {
            next: [0; 26],
            branch: 0,
            cnt: 0,
        }
    }
}
fn insert(trie: &mut Vec<TrieNode>, s: &str) {
    let mut cur = 0;
    for c in s.chars() {
        let i = (c as u8 - b'a') as usize;
        if trie[cur].next[i] == 0 {
            trie.push(TrieNode::new());
            trie[cur].next[i] = trie.len() - 1;
            trie[cur].branch += 1;
        }
        cur = trie[cur].next[i];
        trie[cur].cnt += 1;
    }
    trie[cur].branch += 1;
}

fn dfs(trie: &Vec<TrieNode>, par: usize, cur: usize) -> usize {
    let mut ret = 0;
    if trie[par].branch > 1 {
        ret += trie[cur].cnt;
    }
    (0..26).for_each(|i| {
        if trie[cur].next[i] != 0 {
            ret += dfs(trie, cur, trie[cur].next[i]);
        }
    });
    ret
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace().peekable();

    while let Some(_) = it.peek() {
        let n = next::<usize>(&mut it);
        let mut trie = vec![TrieNode::new()];
        for _ in 0..n {
            insert(&mut trie, &next::<String>(&mut it));
        }

        let mut cnt = 0;
        (0..26).for_each(|i| {
            if trie[0].next[i] != 0 {
                cnt += dfs(&trie, 0, trie[0].next[i]);
            }
        });
        if trie[0].branch == 1 { cnt += n; }
        writeln!(so, "{:.2}", cnt as f64 / n as f64).unwrap();
    }
}
