// BOJ 12583 [Rotate (Small)]
pub fn main() { read();
    for tc in 0..next() {
        let (n, k) = (next::<usize>(), next::<usize>());
        let b = (0..n).map(|_| next::<String>().chars().collect()).collect::<Vec<Vec<_>>>();
        let mut b = (0..n).map(|i| (0..n).map(|j| b[n-j-1][i]).collect::<Vec<_>>()).collect::<Vec<_>>();
        for i in (0..n-1).rev() {
            for j in 0..n {
                if b[i][j] == '.' { continue; }
                for k in (i+1..n).rev() {
                    if b[k][j] == '.' { b[k][j] = b[i][j]; b[i][j] = '.'; break; }
                }
            }
        }

        let (mut rw, mut bw) = (false, false);
        for i in 0..n { for j in 0..n {
            if i+k <= n {
                rw |= (0..k).all(|x| b[i+x][j] == 'R');
                bw |= (0..k).all(|x| b[i+x][j] == 'B');
            }
            if j+k <= n {
                rw |= (0..k).all(|x|b[i][j+x] == 'R');
                bw |= (0..k).all(|x|b[i][j+x] == 'B');
            }
            if i+k <= n && j+k <= n {
                rw |= (0..k).all(|x| b[i+x][j+x] == 'R');
                rw |= (0..k).all(|x| b[i+x][j+k-x-1] == 'R');
                bw |= (0..k).all(|x| b[i+x][j+x] == 'B');
                bw |= (0..k).all(|x| b[i+x][j+k-x-1] == 'B');
            }
            if rw && bw { break; }
        }}

        println!("Case #{}: {}", tc+1, if rw && bw { "Both" } else if rw { "Red" } else if bw { "Blue" } else { "Neither" });
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