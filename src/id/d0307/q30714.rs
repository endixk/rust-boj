// TODO BOJ 30714 [Cartesian]
// Supported by GitHub Copilot

use std::collections::BTreeSet;
fn go(a: &[(i32, i32)], x: i32, k: usize) -> bool {
    let (mut i, mut j) = (0, 0);
    while j < a.len() && a[j].0 - a[i].0 < x { j += 1; }
    let (mut l, mut r) = (BTreeSet::new(), BTreeSet::new());
    (j..a.len()).for_each(|i| { r.insert((a[i].1, a[i].0)); } );
    for j in j..a.len()-1 {
        r.remove(&(a[j].1, a[j].0));
        while a[j+1].0 - a[i].0 >= x {
            l.insert((a[i].1, a[i].0));
            i += 1;
        }
        if l.len() >= k && r.len() >= k {
            let ly = l.iter().nth(k-1).unwrap().0;
            let ry = r.iter().rev().nth(k-1).unwrap().0;
            if ry - ly >= x { return true; }
            let ly = l.iter().rev().nth(k-1).unwrap().0;
            let ry = r.iter().nth(k-1).unwrap().0;
            if ly - ry >= x { return true; }
        }
    }
    false
}
pub fn main() { read();
    let (n, k) = (next::<usize>(), next::<usize>());
    let mut a = (0..n).map(|_| (next::<i32>(), next::<i32>())).collect::<Vec<_>>();
    a.sort_unstable();

    let (mut l, mut r) = (1, 2_000_000);
    while l < r {
        let m = (l + r) / 2;
        if go(&a, m, k) { l = m + 1; }
        else { r = m; }
    }
    println!("{}", l - 1);
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