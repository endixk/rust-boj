// BOJ 6379 [Scramble Sort]
pub fn main() { read();
    let mut i = 0;
    let (mut w, mut wi) = (vec![], vec![]);
    let (mut n, mut ni) = (vec![], vec![]);
    loop {
        let s = next::<String>();
        if s == "." { break; }
        if s.chars().next().unwrap().is_alphabetic() {
            w.push(s.clone()[..s.len()-1].to_string());
            wi.push(i);
        } else {
            n.push(s[..s.len()-1].parse::<i32>().unwrap());
            ni.push(i);
        }
        i += 1;
        if s.ends_with('.') {
            w.sort_unstable_by_key(|s| s.to_lowercase()); n.sort_unstable();
            let mut v = vec![String::new(); i];
            for (a, &j) in w.iter().zip(wi.iter()) { v[j] = a.clone(); }
            for (&a, &j) in n.iter().zip(ni.iter()) { v[j] = a.to_string(); }
            println!("{}.", v.join(", "));
            i = 0; w.clear(); wi.clear(); n.clear(); ni.clear();
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}