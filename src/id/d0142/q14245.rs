// BOJ 14245 [XOR]
// Supported by GitHub Copilot

struct SegTree {
    n: usize,
    v: Vec<u32>,
    l: Vec<bool>,
    lazy: Vec<u32>,
}
impl SegTree {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![0; m<<1], l: vec![false; m<<1], lazy: vec![0; m<<1] }
    }

    fn build(&mut self, a: &[u32]) {
        for i in 0..a.len() {
            self.v[self.n + i] = a[i];
        }
        for i in (1..self.n).rev() {
            self.v[i] = self.v[i<<1] ^ self.v[i<<1|1];
        }
    }

    fn propagate(&mut self, x: usize, s: usize, e: usize) {
        if !self.l[x] { return; }
        if (e - s) & 1 == 0 { self.v[x] ^= self.lazy[x]; }
        if s < e {
            self.l[x<<1] = true;
            self.lazy[x<<1] ^= self.lazy[x];
            self.l[x<<1|1] = true;
            self.lazy[x<<1|1] ^= self.lazy[x];
        }
        self.lazy[x] = 0;
        self.l[x] = false;
    }

    fn update(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize, v: u32) {
        self.propagate(x, s, e);
        if r < s || e < l { return; }
        if l <= s && e <= r {
            self.l[x] = true;
            self.lazy[x] ^= v;
            self.propagate(x, s, e);
        } else {
            let m = (s + e) >> 1;
            self.update(x<<1, s, m, l, r, v);
            self.update(x<<1|1, m+1, e, l, r, v);
            self.v[x] = self.v[x<<1] ^ self.v[x<<1|1];
        }
    }

    fn query(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize) -> u32 {
        self.propagate(x, s, e);
        if r < s || e < l { return 0; }
        if l <= s && e <= r {
            self.v[x]
        } else {
            let m = (s + e) >> 1;
            self.query(x<<1, s, m, l, r) ^ self.query(x<<1|1, m+1, e, l, r)
        }
    }
}
pub fn main() { read();
    let n = next::<usize>();
    let mut a = vec![0; n+1];
    for i in 1..=n { a[i] = next(); }
    let mut seg = SegTree::new(n+1);
    seg.build(&a);

    for _ in 0..next() {
        let q = next::<i8>() == 1;
        if q {
            let (i, j) = (next::<usize>()+1, next::<usize>()+1);
            seg.update(1, 0, seg.n-1, i, j, next::<u32>());
        } else {
            let i = next::<usize>()+1;
            println!("{}", seg.query(1, 0, seg.n-1, i, i));
            // for i in 1..=n { print!("{} ", seg.query(1, 0, seg.n-1, i, i)); } println!();
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