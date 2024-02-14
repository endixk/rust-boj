// BOJ 6601 [Knight Moves]
const DX: [i32; 8] = [-2, -1, 1, 2, 2, 1, -1, -2];
const DY: [i32; 8] = [1, 2, 2, 1, -1, -2, -2, -1];
fn tour(mem: &mut [u8; 1<<12], i: usize, j: usize) -> u8 {
    if mem[i<<6|j] != u8::MAX { return mem[i<<6|j]; }
    let mut q = vec![i];
    let mut d = 1;
    mem[i<<6|i] = 0;
    while !q.is_empty() {
        let mut nq = vec![];
        for &u in &q {
            let (x, y) = ((u&63)>>3, u&7);
            for (&dx, &dy) in DX.iter().zip(DY.iter()) {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx < 0 || nx >= 8 || ny < 0 || ny >= 8 { continue; }
                let v = i<<6|(nx<<3|ny) as usize;
                if mem[v] == u8::MAX { mem[v] = d; nq.push(v); }
            }
        }
        q = nq; d += 1;
    }
    mem[i<<6|j]
}
pub fn main() { read();
    let mut mem = [u8::MAX; 1<<12];
    while peek() {
        let s = next::<String>();
        let (mut fx, mut fy) = (false, false);
        let u = {
            let a = s.chars().nth(0).unwrap() as usize - 'a' as usize;
            let b = s.chars().nth(1).unwrap() as usize - '1' as usize;
            let a = if a > 3 { fx = true; 7 - a } else { a };
            let b = if b > 3 { fy = true; 7 - b } else { b };
            a << 3 | b
        };
        let t = next::<String>();
        let v = {
            let a = t.chars().nth(0).unwrap() as usize - 'a' as usize;
            let b = t.chars().nth(1).unwrap() as usize - '1' as usize;
            let a = if fx { 7 - a } else { a };
            let b = if fy { 7 - b } else { b };
            a << 3 | b
        };
        let d = tour(&mut mem, u, v);
        println!("To get from {} to {} takes {} knight moves.", s, t, d);
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug, iter::Peekable};
static mut BUF: String = String::new();
static mut IT: Option<Peekable<SplitAsciiWhitespace>> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace().peekable());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}
fn peek() -> bool { unsafe { IT.as_mut().unwrap().peek().is_some() } }
