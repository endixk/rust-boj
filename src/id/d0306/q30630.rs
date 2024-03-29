// BOJ 30630 [Left Triangle]
// Supported by GitHub Copilot

#[derive(Eq, PartialEq, Clone, Copy, Debug)] struct Point { x: i32, y: i32, xc: usize, i: usize, q: bool }
static mut ORI: Point = Point { x: 0, y: 0, xc: 0, i: 0, q: true };
impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            ccw(&*std::ptr::addr_of!(ORI), other, self).cmp(&0)
                .then(dsq(&*std::ptr::addr_of!(ORI), self).cmp(&dsq(&*std::ptr::addr_of!(ORI), other)).then(self.q.cmp(&other.q)))
        }
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
fn ccw(a: &Point, b: &Point, c: &Point) -> i8 {
    let t = (b.x - a.x) as i64 * (c.y - a.y) as i64 - (b.y - a.y) as i64 * (c.x - a.x) as i64;
    if t > 0 { 1 } else if t < 0 { -1 } else { 0 }
}
fn dsq(a: &Point, b: &Point) -> i64 {
    ((a.x - b.x) as i64).pow(2) + ((a.y - b.y) as i64).pow(2)
}

// Sum segment tree
struct SegTree<T> {
    n: usize,
    v: Vec<T>,
}
impl<T> SegTree<T> where
    T: std::ops::AddAssign + std::ops::Add<Output=T> + Default + Copy {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![T::default(); m<<1] }
    }
    fn update(&mut self, mut i: usize, x: T) {
        i += self.n;
        self.v[i] += x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1] + self.v[i<<1|1];
        }
    }
    fn query(&mut self, mut l: usize, mut r: usize) -> T {
        l += self.n; r += self.n;
        let mut ans = T::default();
        while l <= r {
            if l & 1 == 1 { ans += self.v[l]; l += 1; }
            if r & 1 == 0 { ans += self.v[r]; r -= 1; }
            l >>= 1; r >>= 1;
        }
        ans
    }
}

pub fn main() { read();
    let n = next::<usize>();
    let mut p = vec![];
    let mut v = vec![];
    for i in 0..n {
        let (x, y) = (next::<i32>(), next::<i32>());
        p.push(Point { x, y, xc: 0, i, q: false });
        v.push((x, i));
    }

    let m = next::<usize>();
    for i in n..n+m {
        let (x, y) = (next::<i32>(), next::<i32>());
        p.push(Point { x, y, xc: 0, i, q: true });
        v.push((x, i));
    }

    v.sort_unstable();
    let (mut x, mut c) = (0, 0);
    for i in 0..n+m {
        if x != v[i].0 { c += 1; x = v[i].0; }
        p[v[i].1].xc = c;
    }

    p.sort_unstable();
    let mut ans = vec![0; m];
    let mut seg = SegTree::<usize>::new(c+1);
    for x in p {
        if x.q {
            ans[x.i-n] = seg.query(0, x.xc);
        } else {
            seg.update(x.xc, 1);
        }
    }

    ans.iter().for_each(|x| println!("{}", x));
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