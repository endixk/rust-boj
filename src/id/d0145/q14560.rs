// BOJ 14560 [Communism]
fn gen(a: &[i64]) -> Vec<i64> {
    let mut v = vec![0];
    for &x in a {
        let (mut i, mut j, mut k) = (0, 0, 0);
        let mut nv = vec![];
        while i < v.len() || j < v.len() || k < v.len() {
            if i < v.len() && (j == v.len() || v[i]-x <= v[j]) && (k == v.len() || v[i]-x <= v[k]+x) { nv.push(v[i]-x); i += 1; }
            if j < v.len() && (i == v.len() || v[j] <= v[i]-x) && (k == v.len() || v[j] <= v[k]+x) { nv.push(v[j]); j += 1; }
            if k < v.len() && (i == v.len() || v[k]+x <= v[i]-x) && (j == v.len() || v[k]+x <= v[j]) { nv.push(v[k]+x); k += 1; }
        }
        v = nv;
    }
    v
}
pub fn main() { read();
    let n = next::<usize>();
    let a = (0..n).map(|_| next::<i64>()).collect::<Vec<_>>();
    let d = next::<i64>();

    let v = gen(&a[..n>>1]);
    let w = gen(&a[n>>1..]);
    let (mut i, mut j, mut ans) = (0, 0, 0);
    for x in v {
        while i < w.len() && w[i] - x < -d { i += 1; }
        while j < w.len() && w[j] - x <= d { j += 1; }
        ans += j - i;
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