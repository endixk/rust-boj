pub fn main() { read();
    let n = next::<usize>();
    let mut v = (0..n).map(|i| (next::<i64>(), i)).collect::<Vec<_>>();
    loop {
        let mut t = Vec::<(i64,usize)>::new();
        let mut flag = false;
        for i in 0..v.len() {
            if flag { flag = false; continue; }
            let mut k = v[i];
            let mut c = 0;
            if !t.is_empty() && t.last().unwrap().0 <= k.0 {
                let (a, _) = t.pop().unwrap();
                c += a;
            }
            if i < v.len()-1 && k.0 >= v[i+1].0 {
                c += v[i+1].0;
                flag = true;
            }
            k.0 += c;
            t.push(k);
        }
        v = t;
        if v.len() == 1 { break; }
    }

    println!("{}\n{}", v[0].0, v[0].1+1);
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