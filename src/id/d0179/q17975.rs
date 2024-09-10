// BOJ 17975 [Strike Zone]
type T = i64;
struct SegTree { n: usize, l: Vec<T>, r: Vec<T>, m: Vec<T>, s: Vec<T> }
impl SegTree {
    fn new(n: usize) -> Self {
        Self {
            n: n.next_power_of_two(),
            l: vec![T::default(); n.next_power_of_two()<<1],
            r: vec![T::default(); n.next_power_of_two()<<1],
            m: vec![T::default(); n.next_power_of_two()<<1],
            s: vec![T::default(); n.next_power_of_two()<<1],
        }
    }
    fn update(&mut self, mut i: usize, x: T) {
        i |= self.n;
        self.l[i] = x; self.r[i] = x; self.m[i] = x; self.s[i] = x;
        while i > 1 {
            i >>= 1;
            self.l[i] = self.l[i<<1].max(self.s[i<<1] + self.l[i<<1|1]);
            self.r[i] = self.r[i<<1|1].max(self.s[i<<1|1] + self.r[i<<1]);
            self.m[i] = self.m[i<<1].max(self.m[i<<1|1]).max(self.r[i<<1] + self.l[i<<1|1]);
            self.s[i] = self.s[i<<1] + self.s[i<<1|1];
        }
    }
}
pub fn main() { read();
    let mut v = vec![];
    let n = next::<usize>();
    let p = (0..n).map(|_| {
        let (x, y) = (next::<i64>(), next::<i64>());
        v.push(x); v.push(y);
        (x, y)
    }).collect::<Vec<_>>();
    let m = next::<usize>();
    let q = (0..m).map(|_| {
        let (x, y) = (next::<i64>(), next::<i64>());
        v.push(x); v.push(y);
        (x, y)
    }).collect::<Vec<_>>();
    let (a, b) = (next::<i64>(), next::<i64>());

    v.sort_unstable(); v.dedup();
    let mut map = std::collections::HashMap::new();
    for i in 0..v.len() { map.insert(v[i], i); }
    let mut p = p.into_iter().map(|(x, y)| (map[&x], map[&y], a)).collect::<Vec<_>>();
    p.extend(q.into_iter().map(|(x, y)| (map[&x], map[&y], -b)));
    p.sort_unstable();

    let mut ans = i64::MIN;
    for i in 0..p.len() {
        let mut seg = SegTree::new(v.len());
        for j in i..p.len() {
            seg.update(p[j].1, p[j].2);
            ans = ans.max(seg.m[1]);
        }
    }
    println!("{}", ans);
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
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