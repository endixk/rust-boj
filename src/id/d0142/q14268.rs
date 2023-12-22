// BOJ 14268 [Company Culture 2]
// Supported by GitHub Copilot

struct FenwickTree {
    n: usize,
    ft: Vec<i64>,
}
impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            ft: vec![0; n+1],
        }
    }
    fn sum(&self, mut i: usize) -> i64 {
        let mut sum = 0;
        while i > 0 {
            sum += self.ft[i];
            i -= i & (!i + 1);
        }
        sum
    }
    fn add(&mut self, mut i: usize, v: i64) {
        while i <= self.n {
            self.ft[i] += v;
            i += i & (!i + 1);
        }
    }

    fn point_update(&mut self, i: usize, x: i64) {
        self.add(i, x);
    }
    fn range_query(&self, l: usize, r: usize) -> i64 {
        self.sum(r) - self.sum(l-1)
    }
    fn range_update(&mut self, l: usize, r: usize, v: i64) {
        self.add(l, v);
        self.add(r+1, -v);
    }
    fn point_query(&self, i: usize) -> i64 {
        self.sum(i)
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
    let (n, m, _) = (next::<usize>(), next::<usize>(), next::<i8>());
    let mut adj = vec![vec![]; n+1];
    for i in 2..=n {
        let p = next::<usize>();
        adj[p].push(i);
    }

    let mut id = vec![0; n+1];
    let mut sz = vec![0; n+1];
    ett(&adj, 1, &mut 1, &mut id, &mut sz);
    let mut ft = FenwickTree::new(n+1);

    for _ in 0..m {
        let q = next::<i8>();
        if q == 1 {
            let (i, w) = (next::<usize>(), next::<i64>());
            ft.range_update(id[i], id[i] + sz[i] - 1, w);
        } else {
            let i = next::<usize>();
            println!("{}", ft.point_query(id[i]));
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