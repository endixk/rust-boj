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

use std::collections::HashMap;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut map = HashMap::new();
    let mut i = 0;
    let mut v = Vec::new();
    let mut names = Vec::new();
    for _ in 0..n {
        let (name, day, start, end) = (next::<String>(&mut it), next::<usize>(&mut it)-1, next::<String>(&mut it), next::<String>(&mut it));
        let k = name.clone();
        let c = map.entry(name).or_insert_with(|| { v.push((vec![0; m/7], vec![0; m/7])); names.push(k.clone()); i += 1; i-1 });
        v[*c].0[day/7] += 1;
        let start = &start[0..2].parse::<usize>().unwrap() * 60 + &start[3..].parse::<usize>().unwrap();
        let end = &end[0..2].parse::<usize>().unwrap() * 60 + &end[3..].parse::<usize>().unwrap();
        v[*c].1[day/7] += end - start;
    }

    let mut ans = Vec::new();
    for j in 0..i {
        if v[j].0.iter().any(|&x| x < 5) { continue; }
        if v[j].1.iter().any(|&x| x < 3600) { continue; }
        ans.push(names[j].clone());
    }
    if ans.is_empty() { writeln!(so, "-1").unwrap(); }
    else {
        ans.sort_unstable();
        for name in ans {
            writeln!(so, "{}", name).unwrap();
        }
    }
}