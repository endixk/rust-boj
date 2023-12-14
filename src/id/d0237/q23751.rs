// BOJ 23751 [Relay Dungeon]
// Supported by GitHub Copilot

struct MinSegTree {
    n: usize,
    v: Vec<i64>,
    lazy: Vec<i64>,
}
const INF: i64 = 0x3f3f3f3f3f3f3f3f;
impl MinSegTree {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![INF; m<<1], lazy: vec![0; m<<1] }
    }

    fn build(&mut self, a: &[i64]) {
        for i in 0..a.len() {
            self.v[self.n + i] = a[i];
        }
        for i in (1..self.n).rev() {
            self.v[i] = self.v[i<<1].min(self.v[i<<1|1]);
        }
    }

    fn propagate(&mut self, x: usize, s: usize, e: usize) {
        if self.lazy[x] == 0 { return; }
        self.v[x] += self.lazy[x];
        if s < e {
            self.lazy[x<<1] += self.lazy[x];
            self.lazy[x<<1|1] += self.lazy[x];
        }
        self.lazy[x] = 0;
    }

    fn update(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize, v: i64) {
        self.propagate(x, s, e);
        if r < s || e < l { return; }
        if l <= s && e <= r {
            self.lazy[x] += v;
            self.propagate(x, s, e);
        } else {
            let m = (s + e) >> 1;
            self.update(x<<1, s, m, l, r, v);
            self.update(x<<1|1, m+1, e, l, r, v);
            self.v[x] = self.v[x<<1].min(self.v[x<<1|1]);
        }
    }

    fn query(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize) -> i64 {
        self.propagate(x, s, e);
        if r < s || e < l { return INF; }
        if l <= s && e <= r {
            self.v[x]
        } else {
            let m = (s + e) >> 1;
            self.query(x<<1, s, m, l, r).min(self.query(x<<1|1, m+1, e, l, r))
        }
    }
}

pub fn main() { read();
    let n = next::<usize>();
    let mut a = vec![(0, 0, 0, 0); n+1];
    for i in 1..=n { a[i] = (next::<i64>(), i, next::<i64>(), next::<i64>()); }
    a.sort_unstable();

    let mut ord = vec![0; n+1];
    for i in 0..=n { ord[a[i].1] = i; }
    let mut lvl = vec![0; n+1];
    for i in 0..=n { lvl[i] = a[i].0; }

    let mut p = vec![0; n+1];
    for i in 1..=n { p[i] = p[i-1] + a[i-1].3 - a[i].2; }

    let mut seg = MinSegTree::new(n+1);
    seg.build(&p);
    for _ in 0..next() {
        // println!("{:?}", (1..=n).map(|i| seg.query(1, 0, n+1, i, i)).collect::<Vec<_>>());
        let q = next::<u8>();
        if q == 1 {
            let (c, l, r) = (next::<i64>(), next::<i64>(), next::<i64>());
            let (l, r) = (lvl.partition_point(|&d| d < l), lvl.partition_point(|&d| d <= r) - 1);
            if r == 0 || l > r { println!("-1"); continue; }

            let p = seg.query(1, 0, seg.n-1, l-1, l-1);
            let m = seg.query(1, 0, seg.n-1, l, r);
            let q = seg.query(1, 0, seg.n-1, r, r);
            if c - p - a[l-1].3 + m < 0 { println!("-1"); continue; }
            println!("1 {}", c - p - a[l-1].3 + q + a[r].3);
        } else if q == 2 {
            let (x, v) = (ord[next::<usize>()], next::<i64>());
            seg.update(1, 0, seg.n-1, x, n, a[x].2 - v);
            a[x].2 = v;
        } else {
            let (x, v) = (ord[next::<usize>()], next::<i64>());
            seg.update(1, 0, seg.n-1, x+1, n, v - a[x].3);
            a[x].3 = v;
        }
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}