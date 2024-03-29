// BOJ 31222 [Sequence and Not Difficult Queries]
struct SegTree<T> {
    n: usize,
    v: Vec<T>,
}
impl<T> SegTree<T> where
    T: std::ops::AddAssign + std::ops::Add<Output=T> + Default + Copy {
    fn new(n: usize) -> Self {
        Self { n: n.next_power_of_two(), v: vec![T::default(); n.next_power_of_two()<<1] }
    }
    fn update(&mut self, mut i: usize, x: T) {
        i |= self.n;
        self.v[i] = x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1] + self.v[i<<1|1];
        }
    }
    fn query(&mut self, mut l: usize, mut r: usize) -> T {
        l |= self.n; r |= self.n;
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
    let (n, q) = (next::<usize>(), next::<usize>());
    let mut a = (0..n).map(|_| next::<u32>()).collect::<Vec<_>>();
    let mut seg = SegTree::<u32>::new(n);
    for i in 1..n {
        if a[i] != a[i-1] { seg.update(i, 1); }
    }

    for _ in 0..q {
        match next::<u8>() {
            1 => {
                let (i, x) = (next::<usize>() - 1, next::<u32>());
                a[i] = x;
                if i > 0 { seg.update(i, if a[i] != a[i-1] { 1 } else { 0 } ); }
                if i < n-1 { seg.update(i+1, if a[i] != a[i+1] { 1 } else { 0 } ); }
            },
            _ => {
                let (l, r) = (next::<usize>() - 1, next::<usize>() - 1);
                let x = if l > 0 && a[l] != a[l-1] { 0 } else { 1 };
                println!("{}", seg.query(l, r) + x);
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