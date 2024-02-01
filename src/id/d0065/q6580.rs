// BOJ 6580 [Quad Tree]
fn go(b: &Vec<Vec<bool>>, i: usize, j: usize, k: usize) {
    let mut x = 0;
    for i in i..i+k { for j in j..j+k {
        if b[i][j] { x += 1; }
    }}
    if x == 0 { print!("W"); }
    else if x == k*k { print!("B"); }
    else {
        print!("Q");
        go(b, i, j, k/2);
        go(b, i, j+k/2, k/2);
        go(b, i+k/2, j, k/2);
        go(b, i+k/2, j+k/2, k/2);
    }
}
pub fn main() { read();
    let (_, _, n) = (next::<String>(), next::<String>(), next::<usize>());
    for _ in 0..8 { next::<String>(); }

    let mut b = vec![vec![false; n]; n];
    for i in 0..n {
        let s = next::<String>();
        let mut j = 0;
        for mut hex in s[..s.len()-1].split(",").map(|x| i32::from_str_radix(&x[2..], 16).unwrap()) {
            for _ in 0..8 {
                b[i][j] = hex & 1 == 1;
                hex >>= 1; j += 1;
            }
        }
    }

    println!("{n}");
    go(&b, 0, 0, n);
    println!();
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