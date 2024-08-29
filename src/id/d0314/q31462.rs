// BOJ 31462 [Triangular Chocolate Packaging (Sweet)]
pub fn main() { read();
    let n = next::<usize>();
    let mut c = vec![vec![false; n]; n];
    for i in 0..n {
        for (j, x) in next::<String>().into_bytes().into_iter().enumerate() {
            c[i][j] = x == b'R';
        }
    }

    let mut d = vec![vec![false; n]; n];
    for i in 0..n-1 {
        for j in 0..=i {
            if !d[i][j] {
                if c[i][j] {
                    if c[i+1][j] && !d[i+1][j] && c[i+1][j+1] && !d[i+1][j+1] {
                        d[i][j] = true; d[i+1][j] = true; d[i+1][j+1] = true;
                    } else {
                        println!("0"); return;
                    }
                } else {
                    if j == i { println!("0"); return; }
                    else if !c[i][j+1] && !d[i][j+1] && !c[i+1][j+1] && !d[i+1][j+1] {
                        d[i][j] = true; d[i][j+1] = true; d[i+1][j+1] = true;
                    } else {
                        println!("0"); return;
                    }
                }
            }
        }
    }
    println!("{}", if d[n-1].iter().any(|&x| !x) { 0 } else { 1 });
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