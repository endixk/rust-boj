pub fn main() { read();
    let n = next::<usize>();
    let mut k = vec![vec![0; n]; n];
    for i in 0..n { for j in 0..n {
        k[i][j] = next::<i32>();
    }}

    let mut p = vec![vec![0; n+1]; n+1];
    for i in 0..n { for j in 0..n {
        p[i+1][j+1] = k[i][j] + p[i+1][j] + p[i][j+1] - p[i][j];
    }}

    for _ in 0..next() {
        let (r1, c1, r2, c2) = (next::<usize>(), next::<usize>(), next::<usize>(), next::<usize>());
        let x = p[r2][c2] - p[r1-1][c2] - p[r2][c1-1] + p[r1-1][c1-1];
        let y = p[r2-1][c2-1] - p[r1][c2-1] - p[r2-1][c1] + p[r1][c1];
        println!("{}", 2 * y - x);
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