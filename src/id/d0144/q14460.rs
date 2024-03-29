// BOJ 14460 [Why Did the Cow Cross the Road III (Platinum)]
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
        if l > r { return T::default(); }
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
fn go(locs: &Vec<(u32, u32, usize)>, seg: &mut SegTree<u32>, s: usize, e: usize, n: usize, k: usize) -> u32 {
    if s >= e { return 0; }
    let m = s + e >> 1;

    let mut l = (s..=m).map(|i| locs[i]).collect::<Vec<_>>();
    let mut r = (m+1..=e).map(|i| locs[i]).collect::<Vec<_>>();
    l.sort_unstable_by_key(|x| x.1);
    r.sort_unstable_by_key(|x| x.1);

    let mut ret = 0;
    let (mut i, mut j) = (0, 0);
    for _ in 0..l.len()+r.len() {
        if j == r.len() || (i < l.len() && l[i].1 < r[j].1) {
            ret += seg.query(1, (k+1).max(l[i].2)-k-1);
            ret += seg.query(l[i].2+k+1, n);
            i += 1;
        } else {
            seg.update(r[j].2, 1);
            j += 1;
        }
    }
    for i in m+1..=e { seg.update(locs[i].2, 0); }
    ret + go(locs, seg, s, m, n, k) + go(locs, seg, m+1, e, n, k)
}
pub fn main() { read();
    let (n, k) = (next::<usize>(), next::<usize>());
    let mut locs = (0..n).map(|i| (0, 0, i+1)).collect::<Vec<_>>();
    for i in 0..n as u32 { locs[next::<usize>()-1].0 = i; }
    for i in 0..n as u32 { locs[next::<usize>()-1].1 = i; }
    locs.sort_unstable();

    let mut seg = SegTree::new(n+1);
    println!("{}", go(&locs, &mut seg, 0, n-1, n, k));
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