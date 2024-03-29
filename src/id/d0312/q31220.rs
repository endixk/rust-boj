// BOJ 31220 [Connected Dominating Set]
fn fill(a: &mut Vec<Vec<bool>>, i: usize, j: usize, x: usize, y: usize) {
    if x < 2 || y < 2 { return; }
    if x <= 3 {
        for j in j..j+y { a[i+1][j] = true; }
    } else if y <= 3 {
        for i in i..i+x { a[i][j+1] = true; }
    } else {
        for j in j..j+y { a[i+1][j] = true; a[i+x-2][j] = true; }
        for i in i+1..i+x-1 { a[i][j+1] = true; a[i][j+y-2] = true; }
        fill(a, i+2, j+2, x-4, y-4);
    }
}
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let mut a = vec![vec![false; m]; n];
    fill(&mut a, 0, 0, n, m);

    println!("YES");
    for i in 0..n {
        for j in 0..m { print!("{}", if a[i][j] { 1 } else { 0 }); }
        println!();
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