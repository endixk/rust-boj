// BOJ 30806 [Intersection Size Sum]
// Supported by GitHub Copilot

const MOD: i64 = 998_244_353;
const MAX: usize = 1_000_001;
fn ncr(n: usize, r: usize, fmod: &[i64], finv: &[i64]) -> i64 { fmod[n] * finv[r] % MOD * finv[n-r] % MOD }
pub fn main() {
    let s = read();
    let mut it = s.split_ascii_whitespace();

    let mut d = vec![1; MAX];
    let mut fmod = vec![1; MAX];
    let mut finv = vec![1; MAX];
    for i in 2..MAX {
        d[i] = -(MOD / i as i64) * d[MOD as usize % i] % MOD;
        fmod[i] = fmod[i-1] * i as i64 % MOD;
        finv[i] = finv[i-1] * d[i] % MOD;
    }

    let n = next::<usize>(&mut it);
    let mut v = vec![];
    for _ in 0..n {
        let k = next::<usize>(&mut it);
        let (mut p, mut c) = (0, 0);
        for _ in 0..k {
            let x = next::<usize>(&mut it);
            if x == p { c += 1; }
            else { p = x; c = 1; }
            v.push((x, c));
        }
    }
    v.sort_unstable();

    let mut w = vec![];
    let (mut p, mut i, mut c) = (v[0].0, v[0].1, 1);
    v.into_iter().skip(1).for_each(|(x, j)| {
        if x == p && j == i { c += 1; }
        else {
            w.push(c);
            p = x; i = j; c = 1;
        }
    });
    w.push(c);

    let mut ans = vec![0; n+1];
    for c in w {
        for x in 1..=c {
            ans[x] = (ans[x] + ncr(c, x, &fmod, &finv) + MOD) % MOD;
        }
    }

    ans.into_iter().skip(1).for_each(|x| println!("{}", x));
}

use std::{io::*, cell::*, str::*, fmt::Debug};
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() -> String {
    let mut s = String::new();
    SI.with(|c| c.borrow_mut().read_to_string(&mut s).unwrap());
    s
}
fn next<T: FromStr>(it: &mut SplitAsciiWhitespace) -> T where <T as FromStr>::Err: Debug {
    it.next().unwrap().parse().unwrap()
}
macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}