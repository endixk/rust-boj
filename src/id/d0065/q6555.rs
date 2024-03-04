// BOJ 6555 [The Sierpinski Fractal]
const UNIT: [[char; 4]; 2] = [[' ', '/', '\\', ' '], ['/', '_', '_', '\\']];
fn build(f: &mut Vec<Vec<char>>, i: usize, j: usize, k: usize) {
    if k == 1 {
        for x in 0..2 {
            for y in 0..4 {
                f[i + x][j + y] = UNIT[x][y];
            }
        }
    } else {
        let l = 1 << (k - 1);
        build(f, i, j + l, k - 1);
        build(f, i + l, j, k - 1);
        build(f, i + l, j + (l << 1), k - 1);
    }
}
pub fn main() { read();
    loop {
        let n = next::<usize>();
        if n == 0 { break; }
        let mut f = vec![vec![' '; 1 << n+1]; 1 << n];
        build(&mut f, 0, 0, n);
        for i in 0..(1 << n) {
            let j = f[i].iter().rposition(|&x| x == '\\').unwrap() + 1;
            println!("{}", f[i].iter().take(j).collect::<String>());
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