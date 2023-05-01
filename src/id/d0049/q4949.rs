// BOJ 4949 [The Balance of the World]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    for s in read(&mut si).split("\n") {
        if s.len() == 1 { break }
        let mut st = Vec::new();
        let mut flag = true;
        for c in s.chars() {
            match c {
                '(' | '[' => st.push(c),
                ')' => if let Some(p) = st.pop() {
                    if p != '(' { flag = false; break }
                } else { flag = false; break },
                ']' => if let Some(p) = st.pop() {
                    if p != '[' { flag = false; break }
                } else { flag = false; break },
                _ => (),
            }
        }

        flag &= st.is_empty();
        writeln!(so, "{}", if flag { "yes" } else { "no" }).unwrap();
    }
}
