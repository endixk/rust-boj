// BOJ 15678 [Water Park]
// Supported by GitHub Copilot

use std::io::{self, Read};

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

// Max segment tree
struct SegTree<T> {
    n: usize,
    v: Vec<T>,
}
impl<T> SegTree<T> where
    T: Ord + Default + Copy {
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
            self.v[i] = self.v[i<<1].max(self.v[(i<<1)+1]);
        }
    }
    fn query(&self, mut l: usize, mut r: usize) -> T {
        l += self.n; r += self.n;
        let mut ans: Option<T> = None;
        while l <= r {
            if l & 1 == 1 {
                if let Some(x) = ans { ans = Some(x.max(self.v[l])); }
                else { ans = Some(self.v[l]); }
                l += 1;
            }
            if r & 1 == 0 {
                if let Some(x) = ans { ans = Some(x.max(self.v[r])); }
                else { ans = Some(self.v[r]); }
                r -= 1;
            }
            l >>= 1; r >>= 1;
        }
        ans.unwrap()
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, d) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let v = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();

    let mut dp = vec![0i64; n];
    let mut seg = SegTree::new(n);

    dp[0] = v[0];
    seg.update(0, v[0]);
    let mut ans = v[0];
    for i in 1..n {
        let l = if i > d { i-d } else { 0 };
        dp[i] = seg.query(l, i-1) + v[i];
        dp[i] = dp[i].max(v[i]);
        seg.update(i, dp[i]);
        ans = ans.max(dp[i]);
    }
    println!("{}", ans);
}
