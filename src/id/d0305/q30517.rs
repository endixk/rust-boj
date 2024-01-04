// BOJ 30517 [Couple Breaker]
// Supported by GitHub Copilot

#[derive(Clone, Copy, Debug)]
struct Cut { s: usize, e: usize, l: usize, r: usize, }
impl Cut { fn size(&self) -> usize { self.e - self.s + 1 } }

fn arrange(cut: &Vec<Cut>, prv: usize, i: usize, n: usize, s: &String, p: &mut Vec<usize>, v: &mut Vec<bool>, c: &mut usize) {
    if i == n {
        println!("{}", s);
        for x in p { print!("{} ", x); } println!();
        *c += 1; if *c == 100 { std::process::exit(0); }
        return;
    }
    for j in 0..n {
        if v[j] { continue; }
        let px = if prv == 0 { 0xffff } else { cut[prv-1].r };
        if px > 0 && px == cut[j].l { continue; }

        v[j] = true; p.push(j+1);
        arrange(cut, j+1, i+1, n, s, p, v, c);
        v[j] = false; p.pop();
    }
}
pub fn main() { read();
    let n = next::<usize>();
    let a = (0..n).map(|_| next::<usize>()).collect::<Vec<_>>();

    let mut cut = vec![];
    let mut p = 0;
    for i in 1..n {
        if a[i] > 0 && a[i-1] == a[i] {
            cut.push(Cut { s: p, e: i-1, l: a[p], r: a[i] });
            p = i;
        }
    }
    cut.push(Cut { s: p, e: n-1, l: a[p], r: a[n-1] });

    let mut c = 0;
    if cut.len() == 2 && cut[0].l > 0 && cut[0].l == cut[1].r {
        if n == 2 { println!("-1"); return; }
        println!("3");
        if cut[0].size() > 1 {
            for p in cut[0].s..cut[0].e {
                let mut ncut = vec![];
                ncut.push(Cut { s: cut[0].s, e: p, l: cut[0].l, r: a[p] });
                ncut.push(Cut { s: p+1, e: cut[0].e, l: a[p+1], r: cut[0].r });
                ncut.push(cut[1]);

                let s = ncut.iter().map(|x| x.size().to_string()).collect::<Vec<_>>().join(" ");
                let mut v = vec![false; ncut.len()];
                let mut p = vec![];
                arrange(&ncut, 0, 0, ncut.len(), &s, &mut p, &mut v, &mut c);
            }
        }
        if cut[1].size() > 1 {
            for p in cut[1].s..cut[1].e {
                let mut ncut = vec![];
                ncut.push(cut[0]);
                ncut.push(Cut { s: cut[1].s, e: p, l: cut[1].l, r: a[p] });
                ncut.push(Cut { s: p+1, e: cut[1].e, l: a[p+1], r: cut[1].r });

                let s = ncut.iter().map(|x| x.size().to_string()).collect::<Vec<_>>().join(" ");
                let mut v = vec![false; ncut.len()];
                let mut p = vec![];
                arrange(&ncut, 0, 0, ncut.len(), &s, &mut p, &mut v, &mut c);
            }
        }
    } else {
        println!("{}", cut.len());
        let s = cut.iter().map(|x| x.size().to_string()).collect::<Vec<_>>().join(" ");
        let mut v = vec![false; cut.len()];
        let mut p = vec![];
        arrange(&cut, 0, 0, cut.len(), &s, &mut p, &mut v, &mut c);
    }

    println!("-1");
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