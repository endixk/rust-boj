// BOJ 31683 [Zlagalica]
pub fn main() { read();
    let n = next::<usize>();
    let mut b = vec![vec!['.'; 200]; 200];

    let a = (0..n).map(|_|
        (next::<char>(), next::<usize>(), next::<usize>(), next::<usize>(), next::<usize>())
    ).collect::<Vec<_>>();

    let (mut xl, mut yl) = (0, 0);
    let (mut x, mut y) = (0, 0);
    for _ in 0..n {
        let k = next::<usize>() - 1;
        for i in x..x+a[k].1 {
            for j in y..y+a[k].2 {
                b[i][j] = a[k].0;
            }
        }
        xl = xl.max(x + a[k].1);
        yl = yl.max(y + a[k].2);
        (x, y) = match a[k].3 {
            0 => (x + a[k].1, y + a[k].4 - 1),
            _ => (x + a[k].1 - a[k].4, y + a[k].2),
        }
    }

    println!("{} {}", xl, yl);
    for i in (0..xl).rev() {
        for j in 0..yl {
            print!("{}", b[i][j]);
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