// BOJ 1665 [Freight Trains]
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

fn get(n: usize, it: &mut std::str::SplitAsciiWhitespace) -> Vec<(usize, i64)> {
    let mut v = Vec::new();
    for _ in 0..n {
        let (i, j) = (next::<usize>(it), next::<usize>(it));
        v.push((j+1, 1));
        v.push((i, -1));
    }
    v.sort_unstable();
    let mut vm = Vec::new();
    for (i, x) in v {
        if vm.is_empty() { vm.push((i, x)); continue; }
        let (j, y) = vm.pop().unwrap();
        if i == j { vm.push((i, x+y)); continue; }
        vm.push((j, y));
        vm.push((i, x));
    }
    vm.iter().filter(|&(_, x)| *x != 0).map(|&(i, x)| (i, x)).collect()
}
fn conv(v: Vec<(usize, i64)>, w: Vec<(usize, i64)>) -> Vec<(usize, i64)> {
    let mut c = Vec::new();
    for &(i, x) in &v {
        for &(j, y) in &w {
            c.push((i+j, x*y));
        }
    }
    c.sort_unstable();
    let mut cm = Vec::new();
    for (i, x) in c {
        if cm.is_empty() { cm.push((i, x)); continue; }
        let (j, y) = cm.pop().unwrap();
        if i == j { cm.push((i, x+y)); continue; }
        cm.push((j, y));
        cm.push((i, x));
    }
    cm.iter().filter(|&(_, x)| *x != 0).map(|&(i, x)| (i, x)).rev().collect()
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let v = get(n, &mut it);
    let m = next::<usize>(&mut it);
    let w = get(m, &mut it);
    let c = conv(v, w);
    let mut p = 0;
    let c = c.into_iter().map(|(i, x)| {
        p += x;
        (i-1, p)
    }).collect::<Vec<_>>();

    let (mut mi, mut m) = (c[0].0-1, c[0].1); p = 0;
    for i in 1..c.len() {
        p += c[i-1].1 * (c[i-1].0 - c[i].0) as i64;
        if m <= p { mi = c[i].0; m = p; }
    }
    writeln!(so, "{}", mi-1).ok();
}
