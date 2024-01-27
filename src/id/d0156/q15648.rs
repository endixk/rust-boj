// BOJ 15648 [Harmony]
struct SegTree {
    n: usize,
    v: Vec<u32>,
}
impl SegTree {
    fn new(n: usize) -> Self {
        Self { n: n.next_power_of_two(), v: vec![0; n.next_power_of_two()<<1] }
    }
    fn update(&mut self, mut i: usize, x: u32) {
        i |= self.n;
        self.v[i] = x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1].max(self.v[i<<1|1]);
        }
    }
    fn query(&self, mut l: usize, mut r: usize) -> u32 {
        l |= self.n; r |= self.n;
        let mut ans = 0;
        while l <= r {
            if l & 1 == 1 { ans = ans.max(self.v[l]); l += 1; }
            if r & 1 == 0 { ans = ans.max(self.v[r]); r -= 1; }
            l >>= 1; r >>= 1;
        }
        ans
    }
}
const MAX: usize = 500_001;
pub fn main() { read();
    let (n, k, d) = (next::<usize>(), next::<usize>(), next::<usize>());
    let a = (0..n).map(|_| next::<u32>()).collect::<Vec<_>>();

    let mut seg = SegTree::new(MAX);
    let mut md = vec![0; k];
    for x in a {
        let a = x as usize;
        let mut m = md[a.checked_rem(k).unwrap()] + 1;
        m = m.max(seg.query(a.saturating_sub(d), if a + d < MAX { a + d } else { MAX - 1 }) + 1);
        md[a.checked_rem(k).unwrap()] = m;
        seg.update(a, m);
    }
    println!("{}", seg.v[1]);
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