// BOJ 24506 [blobpopcorn]
const INF: u32 = 0x3f3f3f3f;
struct SegTree {
    n: usize,
    c: Vec<u32>, // count
    m: Vec<u32>, // max
}
impl SegTree {
    fn new(n: usize) -> Self {
        let n = n.next_power_of_two();
        Self { n, c: vec![0; n<<1], m: vec![0; n<<1] }
    }
    fn gt(&self, i: usize, x: u32) -> u32 {
        if self.m[i] <= x { return 0; }
        if i >= self.n { return if self.m[i] > x { 1 } else { 0 }; }

        return if self.m[i<<1] > x {
            self.gt(i<<1, x) + self.c[i] - self.c[i<<1]
        } else {
            self.gt(i<<1|1, x)
        }
    }
    fn update(&mut self, n: usize, s: usize, e: usize, i: usize, x: u32) {
        if i < s || i > e { return; }
        if s == e { self.c[n] = 1; self.m[n] = x;return; }

        let m = (s+e)>>1;
        self.update(n<<1, s, m, i, x);
        self.update(n<<1|1, m+1, e, i, x);

        let (l, r) = (self.m[n<<1], self.m[n<<1|1]);
        self.c[n] = self.c[n<<1] + self.gt(n<<1|1, l);
        self.m[n] = l.max(r);
    }
}
pub fn main() { read();
    let (n, q) = (next::<usize>(), next::<usize>());
    let mut seg1 = SegTree::new(n);
    let mut seg2 = SegTree::new(n);
    for i in 0..n {
        let x = next::<u32>();
        seg1.update(1, 0, seg1.n-1, i, x);
        seg2.update(1, 0, seg2.n-1, n-i-1, x);
    }
    for _ in 0..q {
        match next::<u8>() {
            2 => println!("{}", ((n as u32)<<1) - seg1.c[1] - seg2.c[1]),
            _ => {
                let (i, x) = (next::<usize>(), next::<u32>());
                seg1.update(1, 0, seg1.n-1, i-1, x);
                seg2.update(1, 0, seg2.n-1, n-i, x);
            },
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