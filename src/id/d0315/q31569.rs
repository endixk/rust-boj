// BOJ 31569 [Triplet XOR and Queries]
struct SegTree {
    n: usize,
    e1: Vec<u64>,
    e2: Vec<u64>,
    e3: Vec<u64>,
    lazy: Vec<u32>,
}
impl SegTree {
    fn new(n: usize) -> Self {
        let m = n.next_power_of_two();
        Self {
            n: m,
            e1: vec![1; m<<1], e2: vec![0; m<<1], e3: vec![0; m<<1],
            lazy: vec![0; m<<1]
        }
    }

    fn propagate(&mut self, x: usize, s: usize, e: usize) {
        if self.lazy[x] == 0 { return; }
        self.e1[x] = self.e1[x].rotate_left(self.lazy[x]);
        self.e2[x] = self.e2[x].rotate_left(self.lazy[x]);
        self.e3[x] = self.e3[x].rotate_left(self.lazy[x]);
        if s < e {
            self.lazy[x<<1] += self.lazy[x];
            self.lazy[x<<1|1] += self.lazy[x];
        }
        self.lazy[x] = 0;
    }

    fn update(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize, v: u32) {
        self.propagate(x, s, e);
        if r < s || e < l { return; }
        if l <= s && e <= r {
            self.lazy[x] += v;
            self.propagate(x, s, e);
        } else {
            let m = (s + e) >> 1;
            self.update(x<<1, s, m, l, r, v);
            self.update(x<<1|1, m+1, e, l, r, v);
            self.e1[x] = self.e1[x<<1] | self.e1[x<<1|1];
            self.e2[x] = self.e2[x<<1] | self.e2[x<<1|1] | (self.e1[x<<1] & self.e1[x<<1|1]);
            self.e3[x] = self.e3[x<<1] | self.e3[x<<1|1] | (self.e1[x<<1] & self.e2[x<<1|1]) | (self.e2[x<<1] & self.e1[x<<1|1])
        }
    }

    fn query(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize) -> (u64, u64, u64) {
        self.propagate(x, s, e);
        if r < s || e < l { return (0, 0, 0); }
        if l <= s && e <= r {
            (self.e1[x], self.e2[x], self.e3[x])
        } else {
            let m = (s + e) >> 1;
            let (a1, a2, a3) = self.query(x<<1, s, m, l, r);
            let (b1, b2, b3) = self.query(x<<1|1, m+1, e, l, r);
            (a1 | b1, a2 | b2 | (a1 & b1), a3 | b3 | (a1 & b2) | (a2 & b1))
        }
    }
}

pub fn main() { read();
    let (n, q) = (next::<usize>(), next::<usize>());
    let mut seg = SegTree::new(n);
    for i in 0..n { seg.update(1, 0, seg.n-1, i, i, next::<u32>()); }

    let mut p = vec![vec![]; 64];
    for i in 0..64 { for j in 0..64 { for k in 0..64 {
        if i == j || i == k || j == k { continue; }
        p[i^j^k].push(1<<i | 1<<j | 1<<k);
    }}}

    for _ in 0..q {
        let (a, l, r, x) = (next::<u8>(), next::<usize>() - 1, next::<usize>() - 1, next::<usize>());
        match a {
            1 => {
                let (a1, a2, a3) = seg.query(1, 0, seg.n-1, l, r);
                if (a3 & (1 << x)) != 0 { println!("1"); continue; }
                if (a2 & !(1 << x)) != 0 && (a1 & (1 << x)) != 0 { println!("1"); continue; }
                println!("{}", if p[x].iter().any(|&y| a1 & y == y) { 1 } else { 0 });
            },
            _ => seg.update(1, 0, seg.n-1, l, r, x as u32),
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