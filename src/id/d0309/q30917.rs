pub fn main() {
    let mut a = 0;
    for i in 1..10 {
        println!("? A {i}");
        SO.with(|c| c.borrow_mut().flush().unwrap());
        if read().parse::<u8>().unwrap() == 1 { a = i; break; }
    }
    let mut b = 0;
    for i in 1..10 {
        println!("? B {i}");
        SO.with(|c| c.borrow_mut().flush().unwrap());
        if read().parse::<u8>().unwrap() == 1 { b = i; break; }
    }
    println!("! {}", a + b);
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
fn read() -> String {
    let mut s = String::new();
    SI.with(|c| c.borrow_mut().read_line(&mut s).unwrap());
    s.trim().to_string()
}