// BOJ 2800 [ZAGRADE]
// Supported by GitHub Copilot

use std::io::{self, BufRead, Write};

fn gen(s: &String, ids: &[u8], bit: u16, n: u8) -> String {
    let bit = (0..n).map(|i| (bit >> i) & 1 == 1).collect::<Vec<_>>();
    let mut ret = String::new();
    for (c, &i) in s.chars().zip(ids.iter()) {
        if i == 0 || bit[i as usize - 1] {
            ret.push(c);
        }
    }
    ret
}

pub fn main() {
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = io::stdin().lock().lines().next().unwrap().unwrap();

    let mut ids = vec![0; s.len()];
    let mut id = 1;
    let mut st = Vec::new();
    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            st.push(i);
            ids[i] = id;
            id += 1;
        } else if c == ')' {
            ids[i] = ids[st.pop().unwrap()];
        }
    }

    let mut v = Vec::new();
    for bit in 1..1 << (id - 1) {
        v.push(gen(&s, &ids, bit - 1, id - 1));
    }
    v.sort_unstable();
    v.dedup();
    v.iter().for_each(|s| writeln!(so, "{}", s).unwrap());
}
