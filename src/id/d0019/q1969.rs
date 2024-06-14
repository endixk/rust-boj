// BOJ 1969 [DNA Consensus String]
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let mut cnt = vec![vec![0; 4]; m];
    for _ in 0..n {
        for (i, c) in next::<String>().chars().into_iter().enumerate() {
            match c {
                'A' => cnt[i][0] += 1,
                'C' => cnt[i][1] += 1,
                'G' => cnt[i][2] += 1,
                 _  => cnt[i][3] += 1,
            }
        }
    }

    let mut ans = String::new();
    let mut ham = 0;
    for v in cnt {
        let max = *v.iter().max().unwrap();
        if max == v[0] { ans.push('A'); }
        else if max == v[1] { ans.push('C'); }
        else if max == v[2] { ans.push('G'); }
        else { ans.push('T'); }
        ham += n - max;
    }

    println!("{}\n{}", ans, ham);
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