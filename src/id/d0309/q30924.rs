fn shuffle(v: &mut Vec<usize>) {
    // generate psuedo-random permutation without using external crate
    let mut rng = 0x3A_94_2C_E1_u32;
    for i in 0..v.len() {
        rng ^= rng << 13;
        rng ^= rng >> 17;
        rng ^= rng << 5;
        v.swap(i, (rng as usize) % (i + 1));
    }
}
pub fn main() {
    let mut v = (1..10001).collect::<Vec<_>>();
    shuffle(&mut v);
    let mut a = 0;
    for &i in &v {
        println!("? A {i}");
        SO.with(|c| c.borrow_mut().flush().unwrap());
        if read().parse::<u8>().unwrap() == 1 { a = i; break; }
    }
    shuffle(&mut v);
    let mut b = 0;
    for &i in &v {
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