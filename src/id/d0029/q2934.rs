// BOJ 2934 [Flowers]
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
pub fn main() { read();
    let mut ft = FenwickTree::new(100_001);
    for _ in 0..next() {
        let (l, r) = (next::<usize>(), next::<usize>());
        let (lx, rx) = (ft.point_query(l), ft.point_query(r));
        println!("{}", lx + rx);
        ft.range_update(l, l, -lx); ft.range_update(r, r, -rx);
        if l+1 <= r-1 { ft.range_update(l+1, r-1, 1); }
    }
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}