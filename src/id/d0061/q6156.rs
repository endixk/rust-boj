// BOJ 6156 [Cow Contest]
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let mut ar = vec![vec![false; n]; n];
    let mut br = vec![vec![false; n]; n];
    for i in 0..n { ar[i][i] = true; br[i][i] = true; }
    for _ in 0..m {
        let (a, b) = (next::<usize>() - 1, next::<usize>() - 1);
        ar[a][b] = true;
        br[b][a] = true;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                ar[i][j] |= ar[i][k] & ar[k][j];
                br[i][j] |= br[i][k] & br[k][j];
            }
        }
    }

    let ac = (0..n).map(|i| ar[i].iter().filter(|&&x| x).count()).collect::<Vec<_>>();
    let bc = (0..n).map(|i| br[i].iter().filter(|&&x| x).count()).collect::<Vec<_>>();
    println!("{}", (0..n).filter(|&i| ac[i] + bc[i] == n + 1).count());
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