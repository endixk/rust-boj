// BOJ 19725 [Checking Answers to Test]
pub fn main() { read();
    let (_, s, m) = (next::<usize>(), next::<String>(), next::<usize>());
    let v = (0..m).map(|_| next::<String>()).collect::<Vec<_>>();

    let mut ans = vec![];
    for i in 0..m-1 { for j in i+1..m {
        let (mut ci, mut wi, mut cmi, mut cwi) = (0, 0, 0, 0);
        let (mut cj, mut wj, mut cmj, mut cwj) = (0, 0, 0, 0);
        for (r, (x, y)) in s.chars().zip(v[i].chars().zip(v[j].chars())) {
            if r == x {
                ci += 1;
                if x == y { cmi += 1; }
            }  else {
                wi += 1;
                if x == y { cwi += 1; }
            }
            if r == y {
                cj += 1;
                if x == y { cmj += 1; }
            }  else {
                wj += 1;
                if x == y { cwj += 1; }
            }
        }

        if ci < cmi * 2 && wi < cwi * 2 && cj < cmj * 2 && wj < cwj * 2 {
            ans.push((i+1, j+1));
        }
    }}

    println!("{}", ans.len());
    for (x, y) in ans { println!("{} {}", x, y); }
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
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