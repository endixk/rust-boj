// BOJ 6560 [Code the Tree]
pub fn main() { read();
    while peek() {
        let s = next::<String>().into_bytes();
        if s.len() == 0 { break; }

        let mut ptr = 0;
        let mut adj= vec![vec![]; 51];
        let mut stk: Vec<usize> = vec![];
        let mut cnt = 0;
        while ptr < s.len() {
            match s[ptr] {
                b' ' | b'(' => ptr += 1,
                b')' => { stk.pop(); ptr += 1; },
                _ => {
                    let mut v = 0;
                    while s[ptr] != b'(' && s[ptr] != b')' && s[ptr] != b' ' {
                        v = v * 10 + (s[ptr] - b'0') as usize;
                        ptr += 1;
                    }
                    if !stk.is_empty() {
                        let u = *stk.last().unwrap();
                        adj[u].push(v); adj[v].push(u);
                    }
                    stk.push(v); cnt += 1;
                }
            }
        }

        for _ in 1..cnt {
            let v = (1..51).filter(|&x| adj[x].len() == 1).min().unwrap();
            let u = adj[v][0];
            print!("{} ", u);
            adj[u].retain(|&x| x != v);
            adj[v].clear();
        }
        println!();
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug, iter::Peekable};
static mut BUF: String = String::new();
static mut IT: Option<Peekable<std::str::Split<'static, char>>> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split('\n').peekable());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}
fn peek() -> bool { unsafe { IT.as_mut().unwrap().peek().is_some() } }
