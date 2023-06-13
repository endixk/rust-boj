// BOJ 7432 [Disk Tree]
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

struct Trie {
    ch: Vec<Trie>,
    cargo: Option<String>,
}
impl Trie {
    fn new() -> Self {
        Trie {
            ch: vec![],
            cargo: None,
        }
    }
    fn push(&mut self, mut v: Vec<String>) {
        if v.is_empty() { return; }
        let s = v.pop().unwrap();
        for child in &mut self.ch {
            if child.cargo == Some(s.clone()) {
                child.push(v);
                return;
            }
        }
        let mut child = Trie::new();
        child.cargo = Some(s.clone());
        child.push(v);
        let loc = self.ch.binary_search_by(|child| child.cargo.as_ref().unwrap().cmp(&s))
            .unwrap_or_else(|x| x);
        self.ch.insert(loc, child);
    }
    fn print(&self, depth: usize, so: &mut dyn Write) {
        if let Some(s) = &self.cargo {
            writeln!(so, "{}{}", " ".repeat(depth-1), s).unwrap();
        }
        for child in &self.ch {
            child.print(depth+1, so);
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut root = Trie::new();
    for _ in 0..next(&mut it) {
        let mut v = next::<String>(&mut it).split("\\").map(|s| s.to_string()).collect::<Vec<_>>();
        v.reverse();
        root.push(v);
    }

    root.print(0, &mut so);
}
