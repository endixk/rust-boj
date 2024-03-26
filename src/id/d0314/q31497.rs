// BOJ 31497 [Happy Birthday]
pub fn main() {
    let n = next().trim().parse::<usize>().unwrap();
    let a = (0..n).map(|_| next()).collect::<Vec<_>>();
    let (mut f, mut log) = (false, 0);
    for i in 0..n {
        let mut k = 0;
        println!("? {}", a[i]); flush();
        k += next().trim().parse::<u8>().unwrap();
        println!("? {}", a[i]); flush();
        k += next().trim().parse::<u8>().unwrap();
        if k == 2 {
            println!("! {}", a[i]);
            return;
        } else if k == 1 {
            if f { println!("! {}", a[i]); return; }
            f = true; log = i;
        }
    }
    println!("! {}", a[log]);
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
use println;
use std::{io::*, cell::*, str::*, fmt::Debug};
static mut BUF: String = String::new();
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn next() -> String { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_line(&mut BUF).unwrap());
    BUF.clone()
}}
fn flush() { SO.with(|c| c.borrow_mut().flush().unwrap()); }