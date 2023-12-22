// BOJ 28928 [Max and Distances]
// Supported by GitHub Copilot

pub fn main() { read();
    let n = next::<usize>();
    let mut arr = vec![vec![0; n]; n];
    let (mut m, mut mi) = (0, 0);
    for i in 0..n { for j in 0..n {
        arr[i][j] = next::<i32>();
        if arr[i][j] > m { (m, mi) = (arr[i][j], i); }
    }}
    if !arr[mi].contains(&0) { println!("NO"); return; }
    let k = arr[mi].iter().position(|&x| x == 0).unwrap();

    let mut v = (0..n).map(|i| (arr[i][k], i)).collect::<Vec<_>>();
    v.sort_unstable();
    let mut a = vec![0; n];
    for (i, &x) in v.iter().enumerate() { a[x.1] = i; }

    let mut w = (0..n).map(|i| (arr[mi][i], i)).collect::<Vec<_>>();
    w.sort_unstable();
    let mut b = vec![0; n];
    for (i, &x) in w.iter().enumerate() { b[x.1] = i; }

    let mut x = vec![0; n];
    let i = a.iter().position(|&x| x == 0).unwrap();
    for j in 0..n { x[b[j]] = arr[i][j]; }

    for i in 0..n { for j in 0..n {
        if arr[i][j] != (x[a[i]] - x[b[j]]).abs() { println!("NO"); return; }
    }}

    println!("YES");
    x.iter().for_each(|&x| print!("{} ", x)); println!();
    a.iter().for_each(|&x| print!("{} ", x+1)); println!();
    b.iter().for_each(|&x| print!("{} ", x+1)); println!();
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