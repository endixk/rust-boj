// BOJ 27921 [Coin Puzzle]
fn cmp(qry: &Vec<Vec<bool>>, nq: usize, mq: usize,
       tgt: &Vec<Vec<bool>>, noff: usize, moff: usize) -> usize {
    let mut ret = 0;
    for i in 0..nq {
        for j in 0..mq {
            ret += if qry[i][j] & tgt[noff + i][moff + j] { 1 } else { 0 };
        }
    }
    ret
}
pub fn main() { read();
    let (nq, mq) = (next::<usize>(), next::<usize>());
    let mut qry = vec![vec![false; mq]; nq];
    let mut cnt = 0;
    for i in 0..nq {
        let s = next::<String>();
        for (j, c) in s.chars().enumerate() {
            if c == 'O' { qry[i][j] = true; cnt += 1; }
        }
    }

    let (nt, mt) = (next::<usize>(), next::<usize>());
    let mut tgt = vec![vec![false; mt + 2*mq]; nt + 2*nq];
    for i in 0..nt {
        let s = next::<String>();
        for (j, c) in s.chars().enumerate() {
            tgt[i+nq][j+mq] = c == 'O';
        }
    }

    let mut ans = 0;
    for i in 0..nt+nq {
        for j in 0..mt+mq {
            ans = ans.max(cmp(&qry, nq, mq, &tgt, i, j));
        }
    }
    println!("{}", cnt - ans);
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}