// TODO BOJ 30800
pub fn main() { read();
    let (h, w) = (next::<usize>(), next::<usize>());
    let (h1, h2, w1, w2) = (next::<usize>(), next::<usize>(), next::<usize>(), next::<usize>());
    let mut c = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            c[i][j] = next::<i64>();
        }
    }

    let mut p = vec![vec![0; w+1]; h+1];
    for i in 0..h {
        for j in 0..w {
            p[i+1][j+1] = p[i+1][j] + p[i][j+1] - p[i][j] + c[i][j];
        }
    }

    let mut cost = 1<<60;
    for x1 in 0..w {
        for x2 in x1+w1..=x1+w2 {
            if x2 >= w { break; }
            for y2 in h1..=h2 {
                if y2 >= h { break; }
                let mut x = p[y2+1][x2+1] - p[y2+1][x1] - p[0][x2+1] + p[0][x1];
                x -= p[y2][x2] - p[y2][x1+1] - p[1][x2] + p[1][x1+1];
                cost = cost.min(x);
            }
        }
    }
    if cost == 1<<60 { println!("No"); }
    else { println!("{}", cost); }
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