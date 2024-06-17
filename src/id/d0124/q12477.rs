// BOJ 12477 [Dire Straights (Small)]
fn score(v: Vec<i32>) -> usize {
    let mut k = 1;
    while v.contains(&(v[0] + k as i32)) { k += 1; }
    if k == 1 { return 1; }
    if k == v.len() { return k; }

    let mut ret = 1;
    for k in 2..=k {
        let mut w = v.clone();
        for x in v[0]..v[0] + k as i32 {
            w.remove(w.iter().position(|&y| y == x).unwrap());
        }
        ret = ret.max(k.min(score(w)));
    }
    ret
}
pub fn main() { read();
    for tc in 0..next() {
        let n = next::<usize>();
        if n == 0 { println!("Case #{}: 0", tc+1); continue; }
        let mut v = (0..n).map(|_| next()).collect::<Vec<i32>>();
        v.sort();
        println!("Case #{}: {}", tc+1, score(v));
    }
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