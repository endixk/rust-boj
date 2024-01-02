// BOJ 16357 [Circuits]
struct MaxSegTree {
    n: usize,
    v: Vec<i32>,
    lazy: Vec<i32>,
}
impl MaxSegTree {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![0; m<<1], lazy: vec![0; m<<1] }
    }

    fn propagate(&mut self, x: usize, s: usize, e: usize) {
        if self.lazy[x] == 0 { return; }
        self.v[x] = self.v[x] + self.lazy[x];
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
            self.v[x] = self.v[x<<1].max(self.v[x<<1|1]);
        }
    }

    fn query(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize) -> i32 {
        self.propagate(x, s, e);
        if r < s || e < l { return 0; }
        if l <= s && e <= r {
            self.v[x]
        } else {
            let m = (s + e) >> 1;
            self.query(x<<1, s, m, l, r).max(self.query(x<<1|1, m+1, e, l, r))
        }
    }
}
pub fn main() { read();
    let n = next::<usize>();
    let mut q = vec![];
    let mut y = vec![];
    for i in 0..n {
        let (_, ye, _, ys) = (next::<i64>(), next::<i64>(), next::<i64>(), next::<i64>());
        q.push((ys, ye));
        y.push((ys, 0, i));
        y.push((ye, 1, i));
    }

    y.sort_unstable();
    let (mut k, mut x) = (y[0].0, 1);
    for &(t, j, i) in &y {
        if t != k { x += 1; k = t; }
        if j == 0 { q[i].0 = x; }
        else { q[i].1 = x; }
    }

    let mut seg = MaxSegTree::new(x as usize);
    for &(s, e) in &q {
        seg.update(1, 0, seg.n-1, s as usize, e as usize, 1);
    }

    let (mut ans, mut k) = (0, 0);
    for (_, j, i) in y {
        if j == 0 {
            seg.update(1, 0, seg.n-1, q[i].0 as usize, q[i].1 as usize, -1);
            k += 1;
        } else { k -= 1; }
        ans = ans.max(k + seg.query(1, 0, seg.n-1, 1, x as usize));
    }
    println!("{}", ans);
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