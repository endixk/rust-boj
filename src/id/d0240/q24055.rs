// BOJ 24055 [Insertion Sort 5]
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

// Sum segment tree
struct SegTree<T> {
    n: usize,
    v: Vec<T>,
}
impl<T> SegTree<T> where
    T: std::ops::AddAssign + std::ops::Add<Output=T> + Default + Copy {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![T::default(); m<<1] }
    }
    fn update(&mut self, mut i: usize, x: T) {
        i += self.n;
        self.v[i] = x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1] + self.v[(i<<1)+1];
        }
    }
    fn query(&mut self, mut l: usize, mut r: usize) -> T {
        l += self.n; r += self.n;
        let mut ans = T::default();
        while l <= r {
            if l & 1 == 1 { ans += self.v[l]; l += 1; }
            if r & 1 == 0 { ans += self.v[r]; r -= 1; }
            l >>= 1; r >>= 1;
        }
        ans
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut v = (0..n as u32).map(|i| (next::<u32>(&mut it), i)).collect::<Vec<_>>();

    v.sort_unstable();
    let map = v.iter().map(|&(x, _)| x).collect::<Vec<_>>();
    let mut val = vec![0; n];
    v.into_iter().enumerate().for_each(|(j, (_, i))| val[i as usize] = j);

    let mut seg = SegTree::<usize>::new(n);
    let mut cnt = 0;
    let mut log = Vec::new();
    for i in 0..n {
        let q = seg.query(val[i]+1, n-1);
        cnt += if q > 0 { q+1 } else { 0 };
        seg.update(val[i], 1);
        log.push(map[val[i]]);

        if k <= cnt {
            log.sort_unstable();
            let x = log[log.binary_search(&map[val[i]]).unwrap()+cnt-k];
            for v in log {
                if v == map[val[i]] && k < cnt { continue; }
                if v == x && k < cnt {
                    write!(so, "{} {} ", v, v).ok();
                } else {
                    write!(so, "{} ", v).ok();
                }
            }
            for j in i+1..n {
                write!(so, "{} ", map[val[j]]).ok();
            }
            writeln!(so).ok();
            return;
        }
    }
    writeln!(so, "-1").ok();
}
