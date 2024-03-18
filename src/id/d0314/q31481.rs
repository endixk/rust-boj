// BOJ 31481 [Sequence and Pranks]
// fast I/O snippet from: https://gist.github.com/kiwiyou/bea8be80e35211fbedc5b780c22ebfe9
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
type I = usize; fn ptr(p: &mut *const u8) -> I { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as I & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}
struct SegTree {
    n: usize,
    v: Vec<i32>,
}
impl SegTree  {
    fn new(n: usize) -> Self {
        Self { n: n.next_power_of_two(), v: vec![0; n.next_power_of_two()<<1] }
    }
    fn update(&mut self, mut i: usize, x: i32) {
        i |= self.n;
        self.v[i] += x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1] + self.v[i<<1|1];
        }
    }
    fn query(&self, k: usize) -> usize {
        let mut i = 1;
        let mut k = k as i32;
        while i < self.n {
            i <<= 1;
            if self.v[i] < k {
                k -= self.v[i];
                i += 1;
            }
        }
        i & !self.n
    }
}
#[inline] fn pick(a: &[i64; 6]) -> (i64, i64, i64) {
    let x = *a.iter().max().unwrap();
    let y = *a.iter().filter(|&&k| k != x).max().unwrap_or(&-1);
    let z = *a.iter().filter(|&&k| k != x && k != y).max().unwrap_or(&-1);
    (x, y, z)
}
struct Lazy {
    n: usize,
    min: Vec<i64>,
    max1: Vec<i64>, max2: Vec<i64>, max3: Vec<i64>,
    lazy: Vec<i64>,
}
impl Lazy {
    fn new(n: usize) -> Self {
        let n = n.next_power_of_two();
        Self { n,
            min: vec![-1; n<<1],
            max1: vec![-1; n<<1], max2: vec![-1; n<<1], max3: vec![-1; n<<1],
            lazy: vec![0; n<<1],
        }
    }

    fn adjust(&mut self, i: usize) {
        self.min[i] = if self.min[i<<1] == -1 { self.min[i<<1|1] } else if self.min[i<<1|1] == -1 { self.min[i<<1] } else { self.min[i<<1].min(self.min[i<<1|1]) };
        (self.max1[i], self.max2[i], self.max3[i]) = pick(&[self.max1[i<<1], self.max2[i<<1], self.max3[i<<1], self.max1[i<<1|1], self.max2[i<<1|1], self.max3[i<<1|1]]);
    }

    fn build(&mut self, a: &[i64]) {
        for i in 0..a.len() {
            self.min[self.n|i] = a[i];
            self.max1[self.n|i] = a[i];
        }
        for i in (1..self.n).rev() {
            self.adjust(i);
        }
    }

    fn propagate(&mut self, x: usize, s: usize, e: usize) {
        if self.lazy[x] == 0 { return; }
        if self.min[x] != -1 { self.min[x] += self.lazy[x]; }
        if self.max1[x] != -1 { self.max1[x] += self.lazy[x]; }
        if self.max2[x] != -1 { self.max2[x] += self.lazy[x]; }
        if self.max3[x] != -1 { self.max3[x] += self.lazy[x]; }
        if s < e {
            self.lazy[x<<1] += self.lazy[x];
            self.lazy[x<<1|1] += self.lazy[x];
        }
        self.lazy[x] = 0;
    }

    fn disable(&mut self, i: usize) {
        (self.min[self.n|i], self.max1[self.n|i], self.max2[self.n|i], self.max3[self.n|i]) = (-1, -1, -1, -1);
        self.update(1, 0, self.n-1, i, i, 0);
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
            self.adjust(x);
        }
    }

    fn query_min(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize) -> i64 {
        self.propagate(x, s, e);
        if r < s || e < l { return -1; }
        if l <= s && e <= r {
            self.min[x]
        } else {
            let m = (s + e) >> 1;
            let p = self.query_min(x<<1, s, m, l, r);
            let q = self.query_min(x<<1|1, m+1, e, l, r);
            if p == -1 { q } else if q == -1 { p } else { p.min(q) }
        }
    }
    fn query_max(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize) -> (i64, i64, i64) {
        self.propagate(x, s, e);
        if r < s || e < l { return (-1, -1, -1); }
        if l <= s && e <= r {
            (self.max1[x], self.max2[x], self.max3[x])
        } else {
            let m = (s + e) >> 1;
            let (a, b, c) = self.query_max(x<<1, s, m, l, r);
            let (d, e, f) = self.query_max(x<<1|1, m+1, e, l, r);
            pick(&[a, b, c, d, e, f])
        }
    }
}
pub fn main() {
    let mut p = input(10333333);
    let n = ptr(&mut p);
    let a = (0..n).map(|_| ptr(&mut p) as i64).collect::<Vec<_>>();

    let mut dseg = SegTree::new(n+1);
    for i in 1..=n { dseg.update(i, 1); }
    let mut lseg = Lazy::new(n);
    lseg.build(&a);
    for _ in 0..ptr(&mut p) {
        let q = ptr(&mut p);
        match q {
            1 => {
                let k = dseg.query(ptr(&mut p));
                dseg.update(k, -1);
                lseg.disable(k-1);
            },
            2 => {
                let (i, r) = (ptr(&mut p), ptr(&mut p));
                let (l, r) = (i - r, i + r);
                let (l, r) = (dseg.query(l)-1, dseg.query(r)-1);
                let x = lseg.query_min(1, 0, lseg.n-1, l, r);
                lseg.update(1, 0, lseg.n-1, l, r, -x);
            },
            3 => {
                let (i, r) = (ptr(&mut p), ptr(&mut p));
                let (l, r) = (i - r, i + r);
                let (l, r) = (dseg.query(l)-1, dseg.query(r)-1);
                let x = lseg.query_max(1, 0, lseg.n-1, l, r).0;
                lseg.update(1, 0, lseg.n-1, l, r, x);
            },
            4 => {
                let (l, r) = (ptr(&mut p), ptr(&mut p));
                let (l, r) = (dseg.query(l)-1, dseg.query(r)-1);
                println!("{}", lseg.query_max(1, 0, lseg.n-1, l, r).2);
            }
            _ => (),
        }
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
use println;
use std::{io::*, cell::*};
thread_local! {
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}