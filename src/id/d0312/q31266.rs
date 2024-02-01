// BOJ 31266 [Soccer Team]
pub fn main() { read();
    let n = next::<usize>();
    let mut v = (0..n).map(|i|
        (next::<i32>(), next::<i32>(), next::<i32>(), next::<i32>(), i)
    ).collect::<Vec<_>>();

    v.sort_unstable_by_key(|&(a, _, _, _, _)| a);
    let pa = (1..=11).map(|i| v[v.len()-i]).collect::<Vec<_>>();
    v.sort_unstable_by_key(|&(_, b, _, _, _)| b);
    let pb = (1..=11).map(|i| v[v.len()-i]).collect::<Vec<_>>();
    v.sort_unstable_by_key(|&(_, _, c, _, _)| c);
    let pc = (1..=11).map(|i| v[v.len()-i]).collect::<Vec<_>>();
    v.sort_unstable_by_key(|&(_, _, _, d, _)| d);
    let pd = (1..=11).map(|i| v[v.len()-i]).collect::<Vec<_>>();

    let mut q = vec![];
    let mut qi = vec![];
    for &(a, b, c, _, i) in &pa { if !qi.contains(&i) { q.push((a.max(b).max(c), i)); qi.push(i); } }
    for &(a, b, c, _, i) in &pb { if !qi.contains(&i) { q.push((a.max(b).max(c), i)); qi.push(i); } }
    for &(a, b, c, _, i) in &pc { if !qi.contains(&i) { q.push((a.max(b).max(c), i)); qi.push(i); } }
    for &(a, b, c, _, i) in &pd { if !qi.contains(&i) { q.push((a.max(b).max(c), i)); qi.push(i); } }
    q.sort_unstable();

    let mut ans = 0;
    for i in 0..11 { for j in 0..11 { for k in 0..11 { for l in 0..11 {
        if pa[i].4 == pb[j].4 || pa[i].4 == pc[k].4 || pa[i].4 == pd[l].4 || pb[j].4 == pc[k].4 || pb[j].4 == pd[l].4 || pc[k].4 == pd[l].4 { continue; }
        let mut x = pa[i].0 + pb[j].1 + pc[k].2 + pd[l].3;

        let mut m = q.len();
        for _ in 0..7 {
            m -= 1;
            while q[m].1 == pa[i].4 || q[m].1 == pb[j].4 || q[m].1 == pc[k].4 || q[m].1 == pd[l].4 { m -= 1; }
            x += q[m].0;
        }
        ans = ans.max(x);
    }}}}
    println!("{}", ans);
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