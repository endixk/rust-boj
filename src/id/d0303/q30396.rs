// BOJ 30396 [Knights' Tour]
// Supported by GitHub Copilot

// 0 1 2 3
// 4 5 6 7
// 8 9 A B
// C D E F
#[inline] fn moves(i: usize) -> Vec<usize> {
    match i {
        0x0 => vec![0x6, 0x9],
        0x1 => vec![0x7, 0x8, 0xA],
        0x2 => vec![0x4, 0x9, 0xB],
        0x3 => vec![0x5, 0xA],
        0x4 => vec![0x2, 0xA, 0xD],
        0x5 => vec![0x3, 0xB, 0xC, 0xE],
        0x6 => vec![0x0, 0x8, 0xD, 0xF],
        0x7 => vec![0x1, 0x9, 0xE],
        0x8 => vec![0x1, 0x6, 0xE],
        0x9 => vec![0x0, 0x2, 0x7, 0xF],
        0xA => vec![0x1, 0x3, 0x4, 0xC],
        0xB => vec![0x2, 0x5, 0xD],
        0xC => vec![0x5, 0xA],
        0xD => vec![0x4, 0x6, 0xB],
        0xE => vec![0x5, 0x7, 0x8],
        0xF => vec![0x6, 0x9],
        _ => unreachable!(),
    }
}
fn go(board: usize) -> Vec<usize> {
    let mut ret = vec![];
    for i in 0..16 {
        if board & (1 << i) == 0 { continue; }
        for m in moves(i) {
            if board & (1 << m) != 0 { continue; }
            ret.push(board ^ (1 << i) ^ (1 << m));
        }
    }
    ret
}
pub fn main() { read();
    let mut a = String::new();
    for _ in 0..4 { a.push_str(next::<String>().as_str()); }
    let a = usize::from_str_radix(&a, 2).unwrap();

    let mut b = String::new();
    for _ in 0..4 { b.push_str(next::<String>().as_str()); }
    let b = usize::from_str_radix(&b, 2).unwrap();

    let mut q = std::collections::VecDeque::new();
    let mut v = vec![false; 1 << 16];
    q.push_back(a); v[a] = true;
    let mut d = 0;
    while !q.is_empty() {
        let sz = q.len();
        for _ in 0..sz {
            let x = q.pop_front().unwrap();
            if x == b { println!("{}", d); return; }
            for y in go(x) {
                if v[y] { continue; }
                v[y] = true;
                q.push_back(y);
            }
        }
        d += 1;
    }
    println!("-1");
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