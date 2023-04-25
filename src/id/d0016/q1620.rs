// BOJ 1620 [Pokémon Master]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let m = it.next().unwrap().parse::<usize>().unwrap();

    let vec = it.by_ref().take(n).collect::<Vec<&str>>();
    let map = vec.iter().enumerate().map(|(i, &s)| (s, i + 1)).collect::<HashMap<_, _>>();
    it.take(m).map(|s| match s.parse::<usize>() {
        Ok(i) => vec[i-1].to_string(),
        Err(_) => map.get(s).unwrap().to_string(),
    }).for_each(|s| writeln!(so, "{}", s).unwrap());
}
