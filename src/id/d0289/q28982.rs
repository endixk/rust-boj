// BOJ 28982 [Sparkling Pluses]
// Supported by GitHub Copilot

pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let mut v = vec![vec![0; m]; n];
    for i in 0..n {
        let s = next::<String>();
        for (j, c) in s.chars().enumerate() {
            v[i][j] = if c == '1' { 1i16 } else { 0i16 };
        }
    }

    let mut l = vec![vec![0; m]; n];
    for i in 0..n {
        l[i][0] = v[i][0];
        for j in 1..m {
            l[i][j] = l[i][j-1] * v[i][j] + v[i][j];
        }
    }
    let mut r = vec![vec![0; m]; n];
    for i in 0..n {
        r[i][m-1] = v[i][m-1];
        for j in (0..m-1).rev() {
            r[i][j] = r[i][j+1] * v[i][j] + v[i][j];
        }
    }
    let mut u = vec![vec![0; m]; n];
    for j in 0..m {
        u[0][j] = v[0][j];
        for i in 1..n {
            u[i][j] = u[i-1][j] * v[i][j] + v[i][j];
        }
    }
    let mut d = vec![vec![0; m]; n];
    for j in 0..m {
        d[n-1][j] = v[n-1][j];
        for i in (0..n-1).rev() {
            d[i][j] = d[i+1][j] * v[i][j] + v[i][j];
        }
    }

    let (mut ans, mut x, mut y) = (0, 0, 0);
    for i in 0..n { for j in 0..m {
        let k = l[i][j].min(r[i][j]).min(u[i][j]).min(d[i][j]) * 4 - 3;
        if ans < k { ans = k; x = i; y = j; }
    }}
    if ans == 0 { println!("-1"); return; }
    println!("{}\n{} {}", ans, x+1, y+1);
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