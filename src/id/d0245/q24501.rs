// BOJ 24501 [blobaww]
const MOD: i64 = 1_000_000_007;
const MAX: usize = 3001;
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let (mut de, mut ds, mut dm) = ([0; MAX], [0; MAX], [0; MAX]);
    let (mut te, mut ts, mut tm) = ([0; MAX], [0; MAX], [0; MAX]);
    let mut flag = false;
    for _ in 0..n {
        let (de, ds, dm, te, ts, tm) = if flag {
            (&mut te, &mut ts, &mut tm, &mut de, &mut ds, &mut dm)
        } else {
            (&mut de, &mut ds, &mut dm, &mut te, &mut ts, &mut tm)
        };
        for (i, c) in next_str().chars().enumerate() {
            te[i+1] = (te[i] + de[i+1] - de[i] + if c == 'E' { 1 } else { 0 }) % MOD;
            ts[i+1] = (ts[i] + ds[i+1] - ds[i] + if c == 'S' { te[i+1] } else { 0 }) % MOD;
            tm[i+1] = (tm[i] + dm[i+1] - dm[i] + if c == 'M' { ts[i+1] } else { 0 }) % MOD;
        }
        flag = !flag;
    }
    println!("{}", (if flag { tm[m] } else { dm[m] } + MOD) % MOD);
}

use std::{io::*, cell::*, str::*, fmt::Debug};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}
fn next_str() -> &'static str {
    unsafe { IT.as_mut().unwrap().next().unwrap() }
}