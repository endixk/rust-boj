// BOJ 30210 [Base Exchange]
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

struct TrieNode {
    child: [usize; 10]
}
fn insert(trie: &mut Vec<TrieNode>, s: &String) {
    let mut cur = 0;
    for c in s.chars() {
        let i = c as usize - '0' as usize;
        if trie[cur].child[i] == 0 {
            trie[cur].child[i] = trie.len();
            trie.push(TrieNode { child: [0; 10] });
        }
        cur = trie[cur].child[i];
    }
}
fn go(trie: &mut Vec<TrieNode>, s: &String) -> usize {
    let mut cur = 0;
    let mut ret = 0;
    for c in s.chars() {
        ret *= 10;
        let i = c as usize - '0' as usize;
        let mut j = 9 - i;
        for _ in 0..10 {
            if trie[cur].child[j] != 0 {
                cur = trie[cur].child[j];
                ret += (i + j) % 10;
                break;
            }
            j = (j + 9) % 10;
        }
    }
    ret
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let a = (0..m).map(|_| next::<String>(&mut it)).collect::<Vec<_>>();
    let b = (0..m).map(|_| next::<String>(&mut it)).collect::<Vec<_>>();

    let mut trie = vec![TrieNode { child: [0; 10] }];
    insert(&mut trie, &format!("{:0>1$}", b[0], n));
    let mut va = 0;
    for i in 1..m {
        va = va.max(go(&mut trie, &format!("{:0>1$}", a[i], n)));
        insert(&mut trie, &format!("{:0>1$}", b[i], n));
    }

    trie = vec![TrieNode { child: [0; 10] }];
    insert(&mut trie, &format!("{:0>1$}", a[0], n));
    let mut vb = 0;
    for i in 1..m {
        insert(&mut trie, &format!("{:0>1$}", a[i], n));
        vb = vb.max(go(&mut trie, &format!("{:0>1$}", b[i], n)));
    }

    writeln!(so, "{}", if va > vb { "J" } else if va == vb { "D" } else { "S" }).ok();

    Ok(())
}
