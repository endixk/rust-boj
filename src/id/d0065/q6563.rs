// BOJ 6563 [Fiber Network]

pub fn main() { read();
    loop {
        let n = next::<usize>();
        if n == 0 { break; }

        let mut adj = [[0u32; 200]; 200];
        loop {
            let (u, v) = (next::<usize>(), next::<usize>());
            if u == 0 { break; }
            let (u, v, s) = (u-1, v-1, next::<String>());
            for c in s.chars() {
                let c = c as usize - 'a' as usize;
                adj[u][v] |= 1 << c;
            }
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    adj[i][j] |= adj[i][k] & adj[k][j];
                }
            }
        }

        loop {
            let (u, v) = (next::<usize>(), next::<usize>());
            if u == 0 { break; }
            let (u, v) = (u-1, v-1);
            let mut ans = String::new();
            for c in 0..26 {
                if (adj[u][v] >> c & 1) == 1 {
                    ans.push((c as u8 + 'a' as u8) as char);
                }
            }
            if ans.is_empty() { println!("-"); }
            else { println!("{}", ans); }
        }
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