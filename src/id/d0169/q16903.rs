// BOJ 16903 [Sequence and Queries 20]
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

#[derive(Default, Clone, Copy)]
struct TrieNode {
    l: usize,
    r: usize,
    cnt: usize,
}
fn insert(trie: &mut Vec<TrieNode>, a: u32) {
    let mut cur = 0;
    for i in (0..30).rev().map(|i| ((a >> i) & 1) as u8) {
        match i == 0 {
            true => {
                if trie[cur].l == 0 {
                    trie[cur].l = trie.len();
                    trie.push(TrieNode::default());
                }
                cur = trie[cur].l;
            }
            false => {
                if trie[cur].r == 0 {
                    trie[cur].r = trie.len();
                    trie.push(TrieNode::default());
                }
                cur = trie[cur].r;
            }
        }
        trie[cur].cnt += 1;
    }
}
fn delete(trie: &mut Vec<TrieNode>, a: u32) {
    let mut cur = 0;
    for i in (0..30).rev().map(|i| ((a >> i) & 1) as u8) {
        cur = if i == 0 { trie[cur].l } else { trie[cur].r };
        trie[cur].cnt -= 1;
    }
}
fn xor(trie: &mut Vec<TrieNode>, x: u32) -> i32 {
    let mut ret = 0;
    let mut cur = 0;
    for i in (0..30).rev().map(|i| ((x >> i) & 1) as u8) {
        match i {
            0 if trie[cur].r != 0 && trie[trie[cur].r].cnt > 0 => {
                ret = (ret << 1) | 1;
                cur = trie[cur].r;
            },
            0 => {
                ret <<= 1;
                cur = trie[cur].l;
            }
            1 if trie[cur].l != 0 && trie[trie[cur].l].cnt > 0 => {
                ret = (ret << 1) | 1;
                cur = trie[cur].l;
            },
            _ => {
                ret <<= 1;
                cur = trie[cur].r;
            }
        }
    }
    ret
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut trie = Vec::new();
    trie.push(TrieNode::default());
    insert(&mut trie, 0);

    for _ in 0..next(&mut it) {
        match next::<u8>(&mut it) {
            1 => insert(&mut trie, next::<u32>(&mut it)),
            2 => delete(&mut trie, next::<u32>(&mut it)),
            _ => writeln!(so, "{}", xor(&mut trie, next::<u32>(&mut it))).unwrap(),
        }
    }
}
