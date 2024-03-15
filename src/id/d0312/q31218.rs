// BOJ 31218 [King of Data Structures]
pub fn main() { read();
    let (n, m, q) = (next::<usize>(), next::<usize>(), next::<usize>());
    let mut b = [true; 1<<20];
    let mut c = n*m;
    for _ in 0..q {
        match next::<u8>() {
            1 => {
                let (dx, dy) = (next::<i32>(), next::<i32>());
                let (mut x, mut y) = (next::<usize>() - 1, next::<usize>() - 1);
                loop {
                    if !b[x<<10|y] { break; }
                    b[x<<10|y] = false; c -= 1;
                    let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                    if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 { break; }
                    (x, y) = (nx as usize, ny as usize);
                }
            },
            2 => println!("{}", if b[next::<usize>() - 1 << 10 | next::<usize>() - 1] { 0 } else { 1 }),
            _ => println!("{c}"),
        }
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