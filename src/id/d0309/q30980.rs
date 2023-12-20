fn proc(p: &mut Vec<Vec<char>>, i: usize, j: usize) {
    let a = p[i+1][j+1] as u8 - '0' as u8;
    let b = p[i+1][j+3] as u8 - '0' as u8;
    let c = p[i+1][j+5] as u8 - '0' as u8;
    let c = if p[i+1][j+6] == '.' { c } else { c*10 + p[i+1][j+6] as u8 - '0' as u8 };

    if a + b == c {
        p[i+1][j] = '*';
        for x in 1..6 { p[i][j+x] = '*'; p[i+2][j+x] = '*'; }
        if c > 9 { p[i][j+6] = '*'; p[i+2][j+6] = '*'; p[i+1][j+7] = '*'; }
        else { p[i+1][j+6] = '*'; }
    } else {
        p[i][j+3] = '/';
        p[i+1][j+2] = '/';
        p[i+2][j+1] = '/';
    }
}
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let mut a = vec![vec![' '; 8*m]; 3*n];
    for i in 0..3*n {
        let s = next::<String>();
        for (j, c) in s.chars().enumerate() {
            a[i][j] = c;
        }
    }

    for i in 0..n { for j in 0..m {
        proc(&mut a, 3*i, 8*j);
    }}

    for i in 0..3*n {
        for j in 0..8*m {
            print!("{}", a[i][j]);
        }
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}