// BOJ 10070 [Wall]
const INF: u32 = 0x3f3f3f3f;
struct SegTree {
    n: usize, l: Vec<u32>, r: Vec<u32>, v: Vec<u32>,
}
impl SegTree {
    fn new(n: usize) -> Self {
        let m = n.next_power_of_two();
        Self { n: m, l: vec![0; m<<1], r: vec![INF; m<<1], v: vec![0; m<<1] }
    }

    fn set(&mut self, p: bool, x: u32, i: usize) {
        if p {
            self.l[i] = self.l[i].max(x);
            self.r[i] = self.r[i].max(x);
        } else {
            self.l[i] = self.l[i].min(x);
            self.r[i] = self.r[i].min(x);
        }
    }
    fn propagate(&mut self, i: usize, s: usize, e: usize) {
        if s < e {
            self.l[i<<1] = self.l[i<<1].max(self.l[i]).min(self.r[i]);
            self.r[i<<1] = self.r[i<<1].max(self.l[i]).min(self.r[i]);
            self.l[i<<1|1] = self.l[i<<1|1].max(self.l[i]).min(self.r[i]);
            self.r[i<<1|1] = self.r[i<<1|1].max(self.l[i]).min(self.r[i]);
        }
        self.l[i] = 0; self.r[i] = INF;
    }

    fn update(&mut self, p: bool, x: u32, i: usize, s: usize, e: usize, l: usize, r: usize) {
        if r < s || e < l { return; }
        if l <= s && e <= r {
            self.set(p, x, i);
            self.v[i] = self.l[i];
        } else {
            self.propagate(i, s, e);
            let m = (s + e) >> 1;
            self.update(p, x, i<<1, s, m, l, r);
            self.update(p, x, i<<1|1, m+1, e, l, r);
        }
    }
}
pub fn main() { read();
    let n = next::<usize>();
    let mut seg = SegTree::new(n);
    for _ in 0..next() {
        let (q, l, r, h) = (next::<u8>() == 1, next::<usize>(), next::<usize>(), next::<u32>());
        seg.update(q, h, 1, 0, seg.n-1, l, r);
    }
    (0..n).for_each(|i| {
        seg.update(true, 0, 1, 0, seg.n-1, i, i);
        println!("{}", seg.v[seg.n|i]);
    })
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