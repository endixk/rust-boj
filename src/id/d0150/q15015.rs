// BOJ 15015 [Manhattan Mornings]
use std::collections::HashMap;
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
    let n = next::<usize>();
    let (sx, sy) = (next::<usize>(), next::<usize>());
    let (ex, ey) = (next::<usize>(), next::<usize>());

    let (mut p, mut x, mut y) = (vec![], vec![], vec![]);
    for _ in 0..n {
        let (mut a, mut b) = (next::<usize>(), next::<usize>());
        if sx <= ex {
            if a < sx || a > ex { continue; }
            a = a - sx;
        } else {
            if a < ex || a > sx { continue; }
            a = sx - a;
        }
        if sy <= ey {
            if b < sy || b > ey { continue; }
            b = b - sy;
        } else {
            if b < ey || b > sy { continue; }
            b = sy - b;
        }
        x.push(a); y.push(b);
        p.push((a, b));
    }

    x.sort_unstable(); x.dedup();
    y.sort_unstable(); y.dedup();
    let (mut xmap, mut ymap) = (HashMap::new(), HashMap::new());
    x.into_iter().enumerate().for_each(|(i, x)| { xmap.insert(x, i); });
    y.into_iter().enumerate().for_each(|(i, y)| { ymap.insert(y, i); });

    for (a, b) in p.iter_mut() {
        *a = xmap[a];
        *b = ymap[b];
    }
    p.sort_unstable();

    let mut ans = 0;
    let mut seg = MaxSegTree::<usize>::new(ymap.len());
    for (_, y) in p {
        let k = seg.query(0, y);
        seg.update(y, k+1);
        ans = ans.max(k+1);
    }

    println!("{}", ans);
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
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