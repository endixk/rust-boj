// BOJ 27922 [Hyundai Mobis Joining Project]
pub fn main() { read();
    let (n, k) = (next::<usize>(), next::<usize>());
    let (mut a, mut b, mut c) = ([0u32; 20001], [0u32; 20001], [0u32; 20001]);
    for _ in 0..n {
        let (x, y, z) = (next::<usize>(), next::<usize>(), next::<usize>());
        a[x+y] += 1; b[y+z] += 1; c[z+x] += 1;
    }
    let (mut x, mut i, mut j) = (0, 20000, k);
    while j > 0 {
        while a[i] == 0 { i -= 1; }
        if a[i] as usize > j { x += i * j; break; }
        x += i * a[i] as usize; j -= a[i] as usize; i -= 1;
    }
    let (mut y, mut i, mut j) = (0, 20000, k);
    while j > 0 {
        while b[i] == 0 { i -= 1; }
        if b[i] as usize > j { y += i * j; break; }
        y += i * b[i] as usize; j -= b[i] as usize; i -= 1;
    }
    let (mut z, mut i, mut j) = (0, 20000, k);
    while j > 0 {
        while c[i] == 0 { i -= 1; }
        if c[i] as usize > j { z += i * j; break; }
        z += i * c[i] as usize; j -= c[i] as usize; i -= 1;
    }
    println!("{}", x.max(y).max(z));
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