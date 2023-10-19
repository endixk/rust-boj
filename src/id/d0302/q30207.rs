// BOJ 30207 [Mess Hall Support]
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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let a = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();
    let sum = a.iter().sum::<i64>();

    let mut b = a.clone();
    let mut pq = std::collections::BinaryHeap::new();
    for i in 0..n {
        pq.push(std::cmp::Reverse((b[i], i)));
    }

    let q = next::<usize>(&mut it);
    let mut qv = (0..q).map(|i| (next::<i64>(&mut it), next::<usize>(&mut it)-1, i)).collect::<Vec<_>>();
    if n == 3 {
        for (d, _, _) in qv {
            write!(so, "{} ", d).unwrap();
        }
        writeln!(so).unwrap();
        return Ok(());
    }

    qv.sort_unstable_by_key(|x| x.0);

    let mut ans = vec![0; q];
    let mut qi = 0;
    for d in 1..50000 {
        let (x, y, z) = (pq.pop().unwrap().0.1, pq.pop().unwrap().0.1, pq.pop().unwrap().0.1);
        b[x] += 1; b[y] += 1; b[z] += 1;
        pq.push(std::cmp::Reverse((b[x], x)));
        pq.push(std::cmp::Reverse((b[y], y)));
        pq.push(std::cmp::Reverse((b[z], z)));
        while qi < q && qv[qi].0 == d {
            ans[qv[qi].2] = b[qv[qi].1] - a[qv[qi].1];
            qi += 1;
        }
    }

    while qi < q {
        let c = sum + qv[qi].0 * 3;
        let off = c - (c / n as i64) * n as i64;
        ans[qv[qi].2] = c / n as i64 + if off > qv[qi].1 as i64 { 1 } else { 0 } - a[qv[qi].1];
        qi += 1;
    }

    ans.iter().for_each(|x| write!(so, "{} ", x).unwrap());
    writeln!(so).unwrap();

    Ok(())
}
