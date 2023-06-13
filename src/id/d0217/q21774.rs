// BOJ 21774 [Log Files]
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

    let (n, q) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut logs = vec![vec![]; 6];
    for _ in 0..n {
        let mut log = next::<String>(&mut it) + &next::<String>(&mut it);
        log = log.replace("-", "").replace(":", "");
        let (ts, lv) = (log[..14].parse::<u64>().unwrap(), log[15..].parse::<usize>().unwrap());
        for i in 0..lv { logs[i].push(ts); }
    }

    for _ in 0..q {
        let mut qry = next::<String>(&mut it) + &next::<String>(&mut it) + &next::<String>(&mut it);
        qry = qry.replace("-", "").replace(":", "");
        let (b, e, l) = (
            qry[..14].parse::<u64>().unwrap(),
            qry[15..29].parse::<u64>().unwrap(),
            qry[30..].parse::<usize>().unwrap());

        let b = logs[l-1].partition_point(|&x| x < b);
        let e = logs[l-1].partition_point(|&x| x <= e);
        writeln!(so, "{}", e-b).unwrap();
    }
}
