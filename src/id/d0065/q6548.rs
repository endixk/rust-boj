// BOJ 6548 [Genetic Code]
fn go(a: &mut Vec<char>, i: usize, n: usize, f: &mut bool) {
    if i == n { *f = true; return; }
    'x: for c in ['N', 'O', 'P'] {
        a[i] = c;
        for x in 1..=(i+1)/2 {
            if &a[i+1-x..i+1] == &a[i+1-2*x..i+1-x] {
                continue 'x;
            }
        }
        go(a, i+1, n, f);
        if *f { return; }
    }
}
pub fn main() { read();
    let mut a = vec!['N'; 5000];
    go(&mut a, 0, 5000, &mut false);
    loop {
        let n: usize = next();
        if n == 0 { break; }
        println!("{}", a[0..n].iter().collect::<String>());
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