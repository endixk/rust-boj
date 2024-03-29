// BOJ 16498 [Smallest Penalty]
#[inline] fn get(a: i32, b: i32, c: i32) -> i32 {
    if a < b {
        if b < c { c - a } else if a < c { b - a } else { b - c }
    } else {
        if a < c { c - b } else if b < c { a - b } else { a - c }
    }
}
pub fn main() { read();
    let (p, q, r) = (next::<usize>(), next::<usize>(), next::<usize>());
    let mut a = (0..p).map(|_| next::<i32>()).collect::<Vec<_>>();
    let mut b = (0..q).map(|_| next::<i32>()).collect::<Vec<_>>();
    let mut c = (0..r).map(|_| next::<i32>()).collect::<Vec<_>>();
    a.sort_unstable(); b.sort_unstable(); c.sort_unstable();

    let mut ans = 0x7fffffff;
    let (mut j, mut k1, mut k2) = (0, 0, 0);
    for i in 0..p {
        while j < q && b[j] < a[i] { j += 1; }
        while k1 < r && c[k1] < a[i] { k1 += 1; }
        while j < q && k2 < r && c[k2] < b[j] { k2 += 1; }
        if j < q && k1 < r { ans = ans.min(get(a[i], b[j], c[k1])); }
        if j < q && k1 > 0 { ans = ans.min(get(a[i], b[j], c[k1 - 1])); }
        if j < q && k2 < r { ans = ans.min(get(a[i], b[j], c[k2])); }
        if j < q && k2 > 0 { ans = ans.min(get(a[i], b[j], c[k2 - 1])); }
        if j > 0 && k1 < r { ans = ans.min(get(a[i], b[j - 1], c[k1])); }
        if j > 0 && k1 > 0 { ans = ans.min(get(a[i], b[j - 1], c[k1 - 1])); }
        if j > 0 && k2 < r { ans = ans.min(get(a[i], b[j - 1], c[k2])); }
        if j > 0 && k2 > 0 { ans = ans.min(get(a[i], b[j - 1], c[k2 - 1])); }
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