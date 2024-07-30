// BOJ 6380 [A Well-Formed Problem]
fn dump() -> (String, bool) {
    let mut r = String::new(); next::<String>();
    let mut s = next::<String>();
    while !s.starts_with("<?") { r += &s; r += " "; s = next(); }
    (r, s == "<?end?>")
}
fn parse(s: String) -> (bool, String, i8) { // (valid, tag, type)
    let l = if s.starts_with("</") { 2 } else { 1 };
    let r = s.len() - if s.ends_with("/>") { 2 } else { 1 };
    let r = if s.contains(" ") { s.find(" ").unwrap() } else { r };
    let v = if s.contains(" ") {
        if !s.contains("=") { false } else {
            let mut b = true;
            if !s.contains("=") { b = false }
            let k = s.split("=").collect::<Vec<_>>();
            let k = k.iter().rev().skip(1).map(|x| x[x.find(" ").unwrap()+1..].to_string()).collect::<Vec<_>>();
            'x: for i in 0..k.len() { for j in i+1..k.len() {
                if k[i] == k[j] { b = false; break 'x; }
            }}
            b
        }
    } else { true };

    (v, s[l..r].to_string(), if s.starts_with("</") { -1 } else if s.ends_with("/>") { 0 } else { 1 })
}
pub fn main() { read();
    next::<String>();
    loop {
        let (s, b) = dump();
        let s = s.as_bytes();
        let (mut f, mut i) = (true, 0);
        let mut stk = vec![];
        let mut emp = true;
        loop {
            while i < s.len() && s[i] != b'<' { i += 1; }
            if i == s.len() { break; }
            let mut j = i;
            while j < s.len() && s[j] != b'>' { j += 1; }
            let (val, tag, cls) = parse(String::from_utf8(s[i..=j].to_vec()).unwrap());
            if !val { f = !f; println!("non well-formed"); break; }
            if cls == -1 {
                if stk.is_empty() || stk.pop().unwrap() != tag { f = !f; println!("non well-formed"); break; }
            } else if stk.contains(&tag) {
                f = !f; println!("non well-formed"); break;
            } else if cls == 1 {
                if stk.is_empty() && !emp { f = !f; println!("non well-formed"); break; }
                stk.push(tag); emp = false;
            }
            i = j + 1;
        }
        if f { println!("{}well-formed", if stk.is_empty() { "" } else { "non " }); }
        if b { break; }
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