pub fn main() { read();
    let (_, s) = (next::<usize>(), next::<String>());
    let mut cnt = vec![0, 0, 0];
    for c in s.chars() {
        if c == 'B' { cnt[0] += 1; }
        else if c == 'S' { cnt[1] += 1; }
        else { cnt[2] += 1; }
    }

    let max = cnt[0].max(cnt[1]).max(cnt[2]);
    if cnt[0] == max && cnt[1] == max && cnt[2] == max { println!("SCU"); }
    else {
        if cnt[0] == max { print!("B"); }
        if cnt[1] == max { print!("S"); }
        if cnt[2] == max { print!("A"); }
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}