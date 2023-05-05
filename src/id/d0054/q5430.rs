// BOJ 5430 [AC]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

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


pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    for _ in 0..next(&mut it) {
        let cmd = next::<String>(&mut it);
        let _ = next::<usize>(&mut it);
        let v = next::<String>(&mut it);
        let mut v = if v.len() > 2 {
            v[1..v.len()-1].split(",").map(|x| x.parse::<u8>().unwrap()).collect::<Vec<_>>()
        } else { vec![] };

        let (mut l, mut r) = (0, v.len() as i32);
        let mut rev = false;
        for c in cmd.chars() {
            if c == 'R' { rev = !rev; }
            else {
                if rev { r -= 1; }
                else { l += 1; }
            }
        }

        if l > r { writeln!(so, "error").ok(); }
        else {
            let res = &mut v[l as usize..r as usize];
            if rev { writeln!(so, "[{}]", res.iter().rev().map(|x| x.to_string()).collect::<Vec<_>>().join(",")).ok(); }
            else { writeln!(so, "[{}]", res.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")).ok(); }
        }
    }
}
