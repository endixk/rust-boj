// BOJ 19585 [Legend]
// Supported by GitHub Copilot

use std::io::{self, BufRead, Write};
use std::collections::HashSet;

fn read<T>(si: &mut T) -> String where T: BufRead {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

struct TrieNode {
    next: [usize; 26],
    end: bool,
}
impl TrieNode {
    fn new() -> Self {
        Self {
            next: [0; 26],
            end: false,
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
        }
        cur = trie[cur].next[i];
    }
    trie[cur].end = true;
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));

    let mut colors = vec![TrieNode::new()];
    (0..n).for_each(|_| { insert(&mut colors, &read(&mut si).to_string()) });
    let mut names = HashSet::new();
    (0..m).for_each(|_| { names.insert(read(&mut si)); });

    for _ in 0..read(&mut si).parse().unwrap() {
        let s = read(&mut si);
        let mut cur = 0;
        let mut fail = true;
        for (i, c) in s.chars().enumerate() {
            let c = (c as u8 - b'a') as usize;
            if colors[cur].next[c] == 0 {
                break;
            }
            cur = colors[cur].next[c];
            if colors[cur].end && names.contains(&s[i + 1..]) {
                fail = false;
                break;
            }
        }
        writeln!(so, "{}", if fail { "No" } else { "Yes" }).unwrap();
    }
}
