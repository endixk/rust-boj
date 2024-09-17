// BOJ 8330 [Permutation]
struct SegTree { n: usize, v: Vec<i32>, lazy: Vec<i32> }
const INF: i32 = 0x3f3f3f3f;
impl SegTree {
    fn new(n: usize) -> Self {
        let m = n.next_power_of_two();
        Self { n: m, v: vec![INF; m<<1], lazy: vec![0; m<<1] }
    }

    fn propagate(&mut self, x: usize, s: usize, e: usize) {
        if self.lazy[x] == 0 { return; }
        if self.v[x] == INF { self.v[x] = 0; }
        self.v[x] += self.lazy[x];
        if s < e {
            self.lazy[x<<1] += self.lazy[x];
            self.lazy[x<<1|1] += self.lazy[x];
        }
        self.lazy[x] = 0;
    }

    fn update(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize, v: i32) {
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

    fn query(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize) -> i32 {
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
pub fn main() {
    let p = &mut input(8444444);
    let n = u32(p) as usize;
    let mut seg = SegTree::new(n+1);
    for i in 1..=n { seg.update(1, 1, seg.n, i, i, i as i32 - 1); }

    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = u32(p) as usize;
        seg.update(1, 1, seg.n, 1, a[i], 1);
    }
    println!("{}", if seg.query(1, 1, seg.n, 1, n) >= n as i32 { "TAK" } else { "NIE" });
    for _ in 0..u32(p) {
        let (i, x) = (u32(p) as usize - 1, u32(p) as usize);
        seg.update(1, 1, seg.n, 1, a[i], -1);
        seg.update(1, 1, seg.n, 1, x, 1);
        a[i] = x;
        println!("{}", if seg.query(1, 1, seg.n, 1, n) >= n as i32 { "TAK" } else { "NIE" });
    }
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
thread_local! {
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
fn u32(p: &mut *const u8) -> u32 { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as u32 & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}