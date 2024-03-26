// BOJ 31503 [DP (Large)]
struct MaxSegTree<T> {
    n: usize,
    v: Vec<T>,
}
impl<T> MaxSegTree<T> where
    T: Ord + Default + Copy {
    fn new(n: usize) -> Self {
        Self { n: n.next_power_of_two(), v: vec![T::default(); n.next_power_of_two()<<1] }
    }
    fn update(&mut self, mut i: usize, x: T) {
        i |= self.n;
        self.v[i] = x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1].max(self.v[i<<1|1]);
        }
    }
    fn query(&self, mut l: usize, mut r: usize) -> T {
        l |= self.n; r |= self.n;
        let mut ans: Option<T> = None;
        while l <= r {
            if l & 1 == 1 {
                if let Some(x) = ans { ans = Some(x.max(self.v[l])); }
                else { ans = Some(self.v[l]); }
                l += 1;
            }
            if r & 1 == 0 {
                if let Some(x) = ans { ans = Some(x.max(self.v[r])); }
                else { ans = Some(self.v[r]); }
                r -= 1;
            }
            l >>= 1; r >>= 1;
        }
        ans.unwrap()
    }
}
pub fn main() { read();
    let (n, q) = (next::<usize>(), next::<usize>());
    let mut a = (0..n).map(|i| (next::<i32>(), i)).collect::<Vec<_>>();
    a.sort_unstable();
    let mut d = vec![0; n];
    let (mut k, mut p) = (0, -1);
    for (x, i) in a {
        if x > p { k += 1; p = x; }
        d[i] = k;
    }

    let mut seg = MaxSegTree::new(n+2);
    let mut f = vec![0; n];
    for (i, &x) in d.iter().enumerate() {
        f[i] = seg.query(0, x-1);
        seg.update(x, f[i] + 1);
    }
    let mut seg = MaxSegTree::new(n+2);
    let mut r = vec![0; n];
    for (i, &x) in d.iter().enumerate().rev() {
        r[i] = seg.query(x+1, n+1);
        seg.update(x, r[i] + 1);
    }

    for _ in 0..q {
        let i = next::<usize>() - 1;
        println!("{}", f[i] + r[i] + 1);
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