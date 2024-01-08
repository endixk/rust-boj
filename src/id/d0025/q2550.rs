// BOJ 2550 [Light Bulbs]
pub fn main() { read();
    let n = next::<usize>();
    let a = (0..n).map(|_| next::<usize>()-1).collect::<Vec<_>>();
    let mut p = vec![0; n];
    for i in 0..n { p[a[i]] = i; }
    let b = (0..n).map(|_| next::<usize>()-1).collect::<Vec<_>>();
    let mut q = vec![0; n];
    for i in 0..n { q[b[i]] = i; }
    let mut v = (0..n).map(|i| (p[i], q[i])).collect::<Vec<_>>();
    v.sort_unstable();

    let mut lis = vec![];
    let mut lix = vec![];
    for &(_, b) in v.iter() {
        if lis.is_empty() || lis.last().unwrap() < &b {
            lis.push(b);
            lix.push(lis.len() - 1);
        } else {
            let i = lis.binary_search(&b).unwrap_or_else(|x| x);
            lis[i] = b;
            lix.push(i);
        }
    }

    let cnt = lis.len();
    let mut trk = vec![false; n];
    for i in 0..n {
        trk[v[i].0] = true;
    }
    let mut i: i32 = cnt as i32 - 1;
    for j in (0..n).rev() {
        if lix[j] == i as usize {
            trk[v[j].0] = false;
            i -= 1;
        }
    }

    println!("{}", cnt);
    let mut x = trk.iter().enumerate().filter(|&(_, &b)| !b).map(|(i, _)| a[i]+1).collect::<Vec<_>>();
    x.sort_unstable();
    x.iter().for_each(|&x| print!("{} ", x));
    println!();
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