// BOJ 3129 [Key]
fn rep(s: String, p: usize) -> bool {
    s.chars().enumerate().all(|(i, c)| c == s.chars().nth(i % p).unwrap())
}
fn gen(sub: &str, t: &str) -> String {
    let mut ret = String::new();
    for (i, j) in sub.chars().zip(t.chars()) {
        ret.push(((i as u8 + 26 - j as u8) % 26 + b'a') as char);
    }
    ret
}
pub fn main() { read();
    let (s, t) = (next::<String>(), next::<String>());
    for i in 0..=s.len() - t.len() {
        for p in 1..=t.len()/2 {
            if rep(gen(&s[i..i+t.len()], &t), p) {
                let key = gen(&s[i..i+t.len()], &t)[..p].to_string();
                for (j, c) in s.chars().enumerate() {
                    print!("{}", ((c as u8 + 26 - key.chars().nth((j + p - i % p) % p).unwrap() as u8) % 26 + b'a' as u8) as char);
                }
                println!();
                return;
            }
        }
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}