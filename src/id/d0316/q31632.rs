// BOJ 31632 [Eggplant Cucumber Carrot]
pub fn main() { read();
    'x: for _ in 0..next() {
        let n = next::<usize>();
        let v = next::<String>().into_bytes();
        let r = next::<String>().into_bytes();

        let g = if v.iter().any(|&c| c == b'G') { 1 } else { 0 };
        let o = if v.iter().any(|&c| c == b'O') { 1 } else { 0 };
        let d = if v.iter().any(|&c| c == b'D') { 1 } else { 0 };
        let t = g + o + d;

        if r.iter().all(|&c| c == b'D') {
            println!("YES");
            match t {
                0 => println!("{}", "G".repeat(n)),
                1 => println!("{}", (if g == 1 { "G" } else if o == 1 { "O" } else { "D" }).repeat(n)),
                2 => println!("{}", v.iter().map(|&c| if c == b'?' { if g == 0 { 'G' } else if o == 0 { 'O' } else { 'D' } } else { c as char }).collect::<String>()),
                _ => println!("{}", v.iter().map(|&c| if c == b'?' { 'G' } else { c as char }).collect::<String>()),
            }
        } else if r.iter().any(|&c| c == b'W') && r.iter().any(|&c| c == b'L') && !r.iter().any(|&c| c == b'D') {
            let (mut w, mut l) = (0, 0);
            for i in 0..n {
                if r[i] == b'W' && v[i] != b'?' { w = v[i]; }
                if r[i] == b'L' && v[i] != b'?' { l = v[i]; }
            }
            if w == 0 && l == 0 { (w, l) = (b'G', b'O'); }
            else if w == 0 { w = if l == b'G' { b'D' } else if l == b'O' { b'G' } else { b'O' }; }
            else if l == 0 { l = if w == b'G' { b'O' } else if w == b'O' { b'D' } else { b'G' }; }

            if w == b'G' && l != b'O' { println!("NO"); continue 'x; }
            if w == b'O' && l != b'D' { println!("NO"); continue 'x; }
            if w == b'D' && l != b'G' { println!("NO"); continue 'x; }

            let mut x = vec![0; n];
            for i in 0..n {
                if v[i] == b'?' { x[i] = if r[i] == b'W' { w } else { l }; }
                else if r[i] == b'W' && v[i] != w { println!("NO"); continue 'x; }
                else if r[i] == b'L' && v[i] != l { println!("NO"); continue 'x; }
                else { x[i] = v[i]; }
            }
            println!("YES"); println!("{}", x.iter().map(|&c| c as char).collect::<String>());
        } else {
            println!("NO");
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