// BOJ 31671 [Exceptional Oreum Hiking]
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let mut h = vec![vec![-1; n+2]; 2*n+1];
    for _ in 0..m {
        let (i, j) = (next::<usize>(), next::<usize>());
        h[i][j] = -2;
    }

    h[0][0] = 0;
    for i in 1..=n { for j in 0..=i {
        if h[i][j] == -2 { continue; }
        let mut k = -1;
        if j > 0 && h[i-1][j-1] >= 0 { k = k.max(h[i-1][j-1]); }
        if h[i-1][j+1] >= 0 { k = k.max(h[i-1][j+1]); }
        if k >= 0 { h[i][j] = k.max(j as i32); }
    }}
    for i in n+1..=2*n { for j in 0..=2*n-i {
        if h[i][j] == -2 { continue; }
        let mut k = -1;
        if j > 0 && h[i-1][j-1] >= 0 { k = k.max(h[i-1][j-1]); }
        if h[i-1][j+1] >= 0 { k = k.max(h[i-1][j+1]); }
        if k >= 0 { h[i][j] = k.max(j as i32); }
    }}
    println!("{}", if h[2*n][0] < 0 { -1 } else { h[2*n][0] });
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