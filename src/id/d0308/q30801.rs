pub fn main() { read();
    let s = next::<String>();
    let s = s.chars().collect::<Vec<_>>();
    let mut v = vec![];
    let mut i = 0;
    while i < s.len() {
        if s[i] == 'L' {
            if s[i+1] == 'U' { v.push(7); }
            else { v.push(1); }
            i += 1;
        } else if s[i] == 'R' {
            if s[i+1] == 'U' { v.push(9); }
            else { v.push(3); }
            i += 1;
        } else if s[i] == 'W' { v.push(8); }
        else if s[i] == 'A' { v.push(4); }
        else if s[i] == 'S' { v.push(2); }
        else if s[i] == 'D' { v.push(6); }
        else {
            match v.pop().unwrap() {
                1 => v.push(9),
                2 => v.push(8),
                3 => v.push(7),
                4 => v.push(6),
                6 => v.push(4),
                7 => v.push(3),
                8 => v.push(2),
                _ => v.push(1),
            }
        }
        i += 1;
    }

    let mut lv = 0;
    let s = next::<String>();
    for c in s.chars() {
        let x = match c {
            'W' => 8,
            'A' => 4,
            'S' => 2,
            'D' => 6,
            c => c.to_digit(10).unwrap() as usize,
        };
        if lv == v.len() {
            lv = 0; continue;
        } else {
            if v[lv] == x { lv += 1; }
            else { lv = 0; }
        }
    }

    println!("{}", if lv == v.len() { "Yes" } else { "No" });
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