// BOJ 16670 [King Kong's Reception]
struct SegTree {
    n: usize,
    v: Vec<i64>, r: Vec<i64>,
}
impl SegTree {
    fn new(n: usize) -> Self {
        let m = n.next_power_of_two();
        let mut r = vec![0; m<<1];
        for i in 0..n { r[m|i] = i as i64; }
        Self {
            n: m, v: vec![0; m<<1], r
        }
    }
    fn update(&mut self, t: usize, d: i64) {
        let mut i = self.n | t;
        self.v[i] += d; self.r[i] += d;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1] + self.v[i<<1|1];
            self.r[i] = self.r[i<<1|1].max(self.r[i<<1] + self.v[i<<1|1])
        }
    }
    fn query(&mut self, t: usize) -> i64 {
        let t = self.n | t;
        let mut ans = 0;
        let mut i = 1;
        for b in (0..self.n.trailing_zeros()).rev() {
            let j = (t >> b) & 1;
            if j == 1 { ans = self.r[i<<1].max(ans + self.v[i<<1]); }
            i = i<<1|j;
        }
        self.r[t].max(ans + self.v[t])
    }
}
pub fn main() { read();
    let mut seg = SegTree::new(2_000_000);
    let mut qry = vec![];
    for _ in 0..next() {
        match next::<char>() {
            '+' => {
                let (t, d) = (next::<usize>(), next::<i64>());
                qry.push((t, d));
                seg.update(t, d);
            },
            '-' => {
                let (t, d) = qry[next::<usize>() - 1];
                qry.push((0, 0));
                seg.update(t, -d);
            },
            _ => {
                let t = next::<usize>();
                qry.push((0, 0));
                let a = seg.query(t).max(t as i64);
                println!("{}", a - t as i64);
            }
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