// BOJ 31481 [Sequence and Pranks]
// TODO FAILED
const INF: i64 = i64::MAX;
struct SegTree<T> {
    n: usize,
    v: Vec<T>,
}
impl<T> SegTree<T> where
    T: std::ops::AddAssign + std::ops::Add<Output=T> + Default + Copy {
    fn new(n: usize) -> Self {
        Self { n: n.next_power_of_two(), v: vec![T::default(); n.next_power_of_two()<<1] }
    }
    fn update(&mut self, mut i: usize, x: T) {
        i |= self.n;
        self.v[i] += x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1] + self.v[i<<1|1];
        }
    }
    fn query(&mut self, mut l: usize, mut r: usize) -> T {
        l |= self.n; r |= self.n;
        let mut ans = T::default();
        while l <= r {
            if l & 1 == 1 { ans += self.v[l]; l += 1; }
            if r & 1 == 0 { ans += self.v[r]; r -= 1; }
            l >>= 1; r >>= 1;
        }
        ans
    }
}
struct MinSegTree {
    n: usize, v: Vec<i64>,
    lazy: Vec<i64>, off: Vec<bool>,
}
impl MinSegTree {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![INF; m<<1], lazy: vec![0; m<<1], off: vec![false; m<<1] }
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
        self.v[x] -= self.lazy[x];
        if s < e {
            self.lazy[x<<1] += self.lazy[x];
            self.lazy[x<<1|1] += self.lazy[x];
        }
        self.lazy[x] = 0;
    }

    fn update(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize, v: i64) {
        if self.off[x] { return; }
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
            if self.off[x] { INF } else { self.v[x] }
        } else {
            let m = (s + e) >> 1;
            self.query(x<<1, s, m, l, r).min(self.query(x<<1|1, m+1, e, l, r))
        }
    }

    fn disable(&mut self, i: usize) { self.off[self.n + i] = true; }
}
struct MaxSegTree { // maintains up to 3 maximum values
    n: usize, v1: Vec<i64>, v2: Vec<i64>, v3: Vec<i64>,
    lazy: Vec<i64>, off: Vec<bool>,
}
impl MaxSegTree {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v1: vec![-1; m<<1], v2: vec![-1; m<<1], v3: vec![-1; m<<1], lazy: vec![0; m<<1], off: vec![false; m<<1] }
    }

    fn build(&mut self, a: &[i64]) {
        for i in 0..a.len() {
            self.v1[self.n + i] = a[i];
        }
        for i in (1..self.n).rev() {
            let mut v = vec![self.v1[i<<1], self.v2[i<<1], self.v3[i<<1], self.v1[i<<1|1], self.v2[i<<1|1], self.v3[i<<1|1]];
            v.sort(); v.dedup(); v.reverse();
            if v.len() > 0 { self.v1[i] = v[0]; }
            if v.len() > 1 { self.v2[i] = v[1]; }
            if v.len() > 2 { self.v3[i] = v[2]; }
        }
    }

    fn propagate(&mut self, x: usize, s: usize, e: usize) {
        if self.lazy[x] == 0 { return; }
        if self.v1[x] == -1 { self.v1[x] = self.lazy[x]; }
        else {
            self.v1[x] += self.lazy[x];
            if self.v2[x] == -1 { self.v2[x] = self.lazy[x]; }
            else {
                self.v2[x] += self.lazy[x];
                if self.v3[x] == -1 { self.v3[x] = self.lazy[x]; }
                else { self.v3[x] += self.lazy[x]; }
            }
        }
        if s < e {
            self.lazy[x<<1] += self.lazy[x];
            self.lazy[x<<1|1] += self.lazy[x];
        }
        self.lazy[x] = 0;
    }

    fn update(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize, v: i64) {
        if self.off[x] { return; }
        self.propagate(x, s, e);
        if r < s || e < l { return; }
        if l <= s && e <= r {
            self.lazy[x] += v;
            self.propagate(x, s, e);
        } else {
            let m = (s + e) >> 1;
            self.update(x<<1, s, m, l, r, v);
            self.update(x<<1|1, m+1, e, l, r, v);
            let mut v = vec![self.v1[x<<1], self.v2[x<<1], self.v3[x<<1], self.v1[x<<1|1], self.v2[x<<1|1], self.v3[x<<1|1]];
            v.sort(); v.dedup(); v.reverse();
            if v.len() > 0 { self.v1[x] = v[0]; }
            if v.len() > 1 { self.v2[x] = v[1]; }
            if v.len() > 2 { self.v3[x] = v[2]; }
        }
    }

    fn query(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize) -> (i64, i64, i64) {
        self.propagate(x, s, e);
        if r < s || e < l { return (-1, -1, -1); }
        return if l <= s && e <= r {
            if self.off[x] { (-1, -1, -1) }
            else { (self.v1[x], self.v2[x], self.v3[x]) }
        } else {
            let m = (s + e) >> 1;
            let (a1, a2, a3) = self.query(x<<1, s, m, l, r);
            let (b1, b2, b3) = self.query(x<<1|1, m+1, e, l, r);
            let mut v = vec![a1, a2, a3, b1, b2, b3];
            v.sort(); v.dedup(); v.reverse();
            return if v.len() > 2 { (v[0], v[1], v[2]) }
            else if v.len() > 1 { (v[0], v[1], -1) }
            else { (v[0], -1, -1) };
        }
    }

    fn disable(&mut self, i: usize) { self.off[self.n + i] = true; }
}
pub fn main() { read();
    let n = next::<usize>();
    let a = (0..n).map(|_| next::<i64>()).collect::<Vec<_>>();

    let mut dseg = SegTree::<usize>::new(n);
    let mut nseg = MinSegTree::new(n);
    let mut xseg = MaxSegTree::new(n);
    nseg.build(&a); xseg.build(&a);
    for _ in 0..next() {
        let q = next::<u8>();
        match q {
            1 => {
                let i = next::<usize>() - 1;
                let k = i + dseg.query(0, i);
                dseg.update(i, 1);
                let x = nseg.query(1, 0, nseg.n-1, k, k);
                nseg.update(1, 0, nseg.n-1, k, k, x+1);
                nseg.disable(k);
                xseg.update(1, 0, xseg.n-1, k, k, -x-1);
                xseg.disable(k);
            },
            2 => {
                let (i, r) = (next::<usize>() - 1, next::<usize>());
                let (l, r) = (i - r, i + r);
                let (l, r) = (l + dseg.query(0, l), r + dseg.query(0, r));
                let x = nseg.query(1, 0, nseg.n-1, l, r);
                nseg.update(1, 0, nseg.n-1, l, r, x);
                xseg.update(1, 0, xseg.n-1, l, r, -x);
            },
            3 => {
                let (i, r) = (next::<usize>() - 1, next::<usize>());
                let (l, r) = (i - r, i + r);
                let (l, r) = (l + dseg.query(0, l), r + dseg.query(0, r));
                let x = xseg.query(1, 0, xseg.n-1, l, r).0;
                xseg.update(1, 0, xseg.n-1, l, r, x);
                nseg.update(1, 0, nseg.n-1, l, r, -x);
            },
            4 => {
                let (l, r) = (next::<usize>() - 1, next::<usize>() - 1);
                let (l, r) = (l + dseg.query(0, l), r + dseg.query(0, r));
                let x = xseg.query(1, 0, xseg.n-1, l, r).2;
                println!("{}", if x < 0 { -1 } else { x });
            }
            _ => (),
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