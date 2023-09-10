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

    let mut map = HashMap::new();
    for _ in 0..next(&mut it) {
        *map.entry(next::<i64>(&mut it)).or_insert(0) += 1;
    }
    for _ in 0..next(&mut it) {
        let a = next::<usize>(&mut it);
        let mut va = (0..a).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();
        let b = next::<usize>(&mut it);
        let vb = (0..b).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();

        va.sort_unstable();
        let mut v = vec![];
        for &x in &va {
            if v.is_empty() { v.push((x, 1)); }
            else if v.last().unwrap().0 == x { v.last_mut().unwrap().1 += 1; }
            else { v.push((x, 1)); }
        }

        let mut flag = true;
        for (x, i) in v {
            if map.get(&x).unwrap_or(&0) < &i { flag = false; break; }
        }
        if flag {
            for x in va {
                *map.get_mut(&x).unwrap() -= 1;
            }
            for x in vb {
                *map.entry(x).or_insert(0) += 1;
            }
        }
    }

    let v = map.iter().map(|(_, &v)| v).sum::<i64>();
    writeln!(so, "{}", v).unwrap();
    if v > 0 {
        for (k, v) in map {
            for _ in 0..v {
                write!(so, "{} ", k).unwrap();
            }
        }
    }
    writeln!(so).unwrap();
}