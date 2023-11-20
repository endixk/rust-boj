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
        self.v[i] += x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1] + self.v[i<<1|1];
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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = (0..n).map(|i| (next::<i32>(&mut it), i)).collect::<Vec<_>>();
    a.sort_unstable();

    let mut b = vec![0; n];
    let (mut c, mut x) = (1, a[0].0);
    for i in 0..n {
        if a[i].0 != x { c += 1; x = a[i].0; }
        b[a[i].1] = c;
    }

    let mut seg = SegTree::new(c+1);
    let mut cnt = 0i64;
    for i in 0..k {
        cnt += seg.query(1, b[i]-1);
        seg.update(b[i], 1);
    }

    let mut ans = cnt;
    for i in k..n {
        cnt -= seg.query(b[i-k]+1, c);
        seg.update(b[i-k], -1);
        cnt += seg.query(1, b[i]-1);
        seg.update(b[i], 1);
        ans = ans.max(cnt);
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}