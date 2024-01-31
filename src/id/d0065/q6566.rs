// BOJ 6566 [Anagram Groups]
pub fn main() { read();
    let mut map = std::collections::HashMap::<String, usize>::new();
    let mut v: Vec<Vec<String>> = Vec::new();
    while peek() {
        let s = next::<String>();
        let mut t = s.clone().into_bytes();
        t.sort_unstable();
        let t = t.into_iter().map(|c| c as char).collect::<String>();
        if let Some(&i) = map.get(&t) {
            v[i].push(s);
        } else {
            map.insert(t, v.len());
            v.push(vec![s]);
        }
    }

    v.iter_mut().for_each(|v| v.sort_unstable());
    v.sort_unstable_by(|a, b| b.len().cmp(&a.len()).then(a[0].cmp(&b[0])));
    for i in 0..5.min(v.len()) {
        let mut w = v[i].clone(); w.dedup();
        println!("Group of size {}: {} .", v[i].len(), w.join(" "));
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug};
static mut BUF: String = String::new();
static mut IT: Option<std::iter::Peekable<SplitAsciiWhitespace>> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace().peekable());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}
fn peek() -> bool {
    unsafe { IT.as_mut().unwrap().peek().is_some() }
}