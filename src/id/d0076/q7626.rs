// BOJ 7626 [Rectangles]
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

// Count segment tree
struct CountSegTree {
    n: usize,
    c: Vec<i32>,
    v: Vec<usize>,
}
impl CountSegTree {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, c: vec![0; m<<1], v: vec![0; m<<1] }
    }
    fn update(&mut self, i: usize, s: usize, e: usize, l: usize, r: usize, x: i32, y: &[usize]) {
        if r < s || e < l { return; }
        if l <= s && e <= r {
            self.c[i] += x;
        } else {
            let m = (s + e) >> 1;
            self.update(i<<1, s, m, l, r, x, y);
            self.update((i<<1)+1, m+1, e, l, r, x, y);
        }
        if self.c[i] > 0 {
            self.v[i] = y[e] - y[s-1];
        } else {
            if s == e { self.v[i] = 0; }
            else { self.v[i] = self.v[i<<1] + self.v[i<<1|1]; }
        }
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let (mut p, mut y) = (vec![], vec![]);
    for _ in 0..n {
        let (x1, x2, y1, y2) = (
            next::<usize>(&mut it), next::<usize>(&mut it),
            next::<usize>(&mut it), next::<usize>(&mut it)
        );
        p.push((x1, y1, y2, true));
        p.push((x2, y1, y2, false));
        y.push(y1);
        y.push(y2);
    }

    p.sort_unstable();
    y.sort_unstable(); y.dedup();
    let mut map = std::collections::HashMap::new();
    for (i, &y) in y.iter().enumerate() {
        map.insert(y, i+1);
    }
    for t in p.iter_mut() {
        t.1 = *map.get(&t.1).unwrap();
        t.2 = *map.get(&t.2).unwrap();
    }

    let (mut a, mut s) = (vec![0], 0);
    for i in 0..y.len()-1 {
        s += y[i+1] - y[i];
        a.push(s);
    }

    let mut seg = CountSegTree::new(y.len()+1);
    let (mut ans, mut prv) = (0, 0);
    for (x, l, r, f) in p {
        ans += seg.v[1] * (x - prv);
        seg.update(1, 1, y.len(), l, r-1, if f { 1 } else { -1 }, &a);
        prv = x;
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
