// BOJ 22243 [Pile of Books]
pub fn main() { read();
    let n = next::<usize>();
    let mut h = vec![vec![0; n]; n];
    for i in 0..n { for j in 0..n {
        h[i][j] = next::<u16>();
    }}

    let mut ans = 0;
    for i in 0..n { for j in 0..n {
        if h[i][j] == 0 { continue; }
        if i == 0 || i == n-1 || j == 0 || j == n-1 { ans += 1; }
        else if (0..i).all(|x| h[x][j] < h[i][j]) { ans += 1; }
        else if (i+1..n).all(|x| h[x][j] < h[i][j]) { ans += 1; }
        else if (0..j).all(|x| h[i][x] < h[i][j]) { ans += 1; }
        else if (j+1..n).all(|x| h[i][x] < h[i][j]) { ans += 1; }
    }}

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