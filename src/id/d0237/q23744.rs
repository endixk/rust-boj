// BOJ 23744 [Algorithm Tutoring]
// Supported by GitHub Copilot

struct MaxSegTree<T> {
    n: usize,
    v: Vec<T>,
}
impl<T> MaxSegTree<T> where
    T: Ord + Default + Copy {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![T::default(); m<<1] }
    }
    fn update(&mut self, mut i: usize, x: T) {
        i += self.n;
        self.v[i] = x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1].max(self.v[i<<1|1]);
        }
    }
    fn query(&self, mut l: usize, mut r: usize) -> T {
        l += self.n; r += self.n;
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

struct MinSegTree<T> {
    n: usize,
    v: Vec<T>,
}
impl<T> MinSegTree<T> where
    T: Ord + Default + Copy {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![T::default(); m<<1] }
    }
    fn update(&mut self, mut i: usize, x: T) {
        i += self.n;
        self.v[i] = x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1].min(self.v[i<<1|1]);
        }
    }
    fn query(&self, mut l: usize, mut r: usize) -> T {
        l += self.n; r += self.n;
        let mut ans: Option<T> = None;
        while l <= r {
            if l & 1 == 1 {
                if let Some(x) = ans { ans = Some(x.min(self.v[l])); }
                else { ans = Some(self.v[l]); }
                l += 1;
            }
            if r & 1 == 0 {
                if let Some(x) = ans { ans = Some(x.min(self.v[r])); }
                else { ans = Some(self.v[r]); }
                r -= 1;
            }
            l >>= 1; r >>= 1;
        }
        ans.unwrap()
    }
}

pub fn main() { read();
    let n = next::<usize>();
    let mut loc = vec![(0, 0, 0)];
    let mut mark = vec![vec![]; n+1];
    for i in 1..=n {
        let (a, l, r) = (next::<i64>(), next::<usize>(), next::<usize>());
        loc.push((a, l, r));
        mark[if i > r { i-r } else { 0 }].push((i, true));
        mark[if i >= l { i-l+1 } else { 0 }].push((i, false));
        if i+l <= n {
            mark[i+l].push((i, true));
            if i+r+1 <= n {
                mark[i+r+1].push((i, false));
            }
        }
    }

    const INF: i64 = 0x3f3f3f3f;
    let mut mint = MinSegTree::new(n+2);
    let mut maxt = MaxSegTree::new(n+2);
    for i in 0..=n+1 {
        mint.update(i, INF);
        maxt.update(i, -INF);
    }

    let mut ans = -1;
    for i in 1..=n {
        for &(x, c) in &mark[i] {
            if c {
                mint.update(x, loc[x].0);
                maxt.update(x, loc[x].0);
            } else {
                mint.update(x, INF);
                maxt.update(x, -INF);
            }
        }

        let (a, l, r) = loc[i];
        let (p, q) = (if i > r { i - r } else { 0 }, if i > l { i - l } else { 0 });
        let (mn, mx) = (mint.query(p, q), maxt.query(p, q));
        let (p, q) = (if i + l <= n { i + l } else { n+1 }, if i + r <= n { i + r } else { n+1 });
        let (mn, mx) = (mn.min(mint.query(p, q)), mx.max(maxt.query(p, q)));

        if mn != INF { ans = ans.max(a - mn); }
        if mx != -INF { ans = ans.max(mx - a); }
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}