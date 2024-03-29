// BOJ 1395 [Light Switching]
struct SegTree {
    n: usize,
    v: Vec<i32>,
    lazy: Vec<bool>,
}
impl SegTree {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![0; m<<1], lazy: vec![false; m<<1] }
    }

    fn propagate(&mut self, x: usize, s: usize, e: usize) {
        if !self.lazy[x] { return; }
        self.v[x] = (e - s + 1) as i32 - self.v[x];
        if s < e {
            self.lazy[x<<1] = !self.lazy[x<<1];
            self.lazy[x<<1|1] = !self.lazy[x<<1|1];
        }
        self.lazy[x] = false;
    }

    fn update(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize) {
        self.propagate(x, s, e);
        if r < s || e < l { return; }
        if l <= s && e <= r {
            self.lazy[x] = !self.lazy[x];
            self.propagate(x, s, e);
        } else {
            let m = (s + e) >> 1;
            self.update(x<<1, s, m, l, r);
            self.update(x<<1|1, m+1, e, l, r);
            self.v[x] = self.v[x<<1] + self.v[x<<1|1];
        }
    }

    fn query(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize) -> i32 {
        self.propagate(x, s, e);
        if r < s || e < l { return 0; }
        if l <= s && e <= r {
            self.v[x]
        } else {
            let m = (s + e) >> 1;
            self.query(x<<1, s, m, l, r) + self.query(x<<1|1, m+1, e, l, r)
        }
    }
}
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let mut seg = SegTree::new(n+1);
    for _ in 0..m {
        let (q, s, t) = (next::<i8>() == 1, next::<usize>(), next::<usize>());
        if q {
            println!("{}", seg.query(1, 1, seg.n, s, t));
        } else {
            seg.update(1, 1, seg.n, s, t);
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}