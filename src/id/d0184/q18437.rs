// BOJ 18437 [Company Culture 5]
// Supported by GitHub Copilot

struct SegTree {
    n: usize,
    v: Vec<usize>,
    lazy: Vec<usize>,
}
impl SegTree {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![0; m<<1], lazy: vec![2; m<<1] }
    }

    fn build(&mut self, a: &[usize]) {
        for i in 0..a.len() {
            self.v[self.n + i] = a[i];
        }
        for i in (1..self.n).rev() {
            self.v[i] = self.v[i<<1] + self.v[i<<1|1];
        }
    }

    fn propagate(&mut self, x: usize, s: usize, e: usize) {
        if self.lazy[x] == 2 { return; }
        self.v[x] = self.lazy[x] * (e - s + 1);
        if s < e {
            self.lazy[x<<1] = self.lazy[x];
            self.lazy[x<<1|1] = self.lazy[x];
        }
        self.lazy[x] = 2;
    }

    fn update(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize, v: usize) {
        self.propagate(x, s, e);
        if r < s || e < l { return; }
        if l <= s && e <= r {
            self.lazy[x] = v;
            self.propagate(x, s, e);
        } else {
            let m = (s + e) >> 1;
            self.update(x<<1, s, m, l, r, v);
            self.update(x<<1|1, m+1, e, l, r, v);
            self.v[x] = self.v[x<<1] + self.v[x<<1|1];
        }
    }

    fn query(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize) -> usize {
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
fn ett(adj: &Vec<Vec<usize>>, cur: usize, x: &mut usize,
       id: &mut [usize], sz: &mut [usize]) -> usize {
    id[cur] = *x; *x += 1;
    sz[cur] = 1;
    for &nxt in adj[cur].iter() {
        sz[cur] += ett(adj, nxt, x, id, sz);
    }
    sz[cur]
}
pub fn main() { read();
    let (n, _) = (next::<usize>(), next::<i8>());
    let mut adj = vec![vec![]; n+1];
    for i in 2..=n {
        let p = next::<usize>();
        adj[p].push(i);
    }

    let mut id = vec![0; n+1];
    let mut sz = vec![0; n+1];
    ett(&adj, 1, &mut 1, &mut id, &mut sz);

    let mut seg = SegTree::new(n+1);
    seg.build(&(0..=n).map(|_| 1).collect::<Vec<_>>());
    for _ in 0..next() {
        let (q, i) = (next::<i8>(), next::<usize>());
        if q == 1 {
            seg.update(1, 1, seg.n, id[i] + 1, id[i] + sz[i] - 1, 1);
        } else if q == 2 {
            seg.update(1, 1, seg.n, id[i] + 1, id[i] + sz[i] - 1, 0);
        } else {
            println!("{}", seg.query(1, 1, seg.n, id[i] + 1, id[i] + sz[i] - 1));
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