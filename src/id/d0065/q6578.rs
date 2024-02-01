// BOJ 6578 [Euro Cup 2000]
pub fn main() { read();
    for tc in 1.. {
        let n = next::<usize>();
        if n == 0 { break; }

        let mut map = std::collections::HashMap::new();
        let mut teams = vec![];
        for i in 0..n {
            let s = next::<String>();
            teams.push(s.clone());
            map.insert(s, i);
        }
        let mut pts = vec![0; n];
        let mut games = vec![vec![false; n]; n];
        for _ in 0..next() {
            let (a, b, c, d) = (next::<String>(), next::<String>(), next::<usize>(), next::<usize>());
            let (a, b) = (map[&a], map[&b]);
            games[a][b] = true;
            if c > d { pts[a] += 3; }
            else if c < d { pts[b] += 3; }
            else { pts[a] += 1; pts[b] += 1; }
        }

        // remaining games
        let mut rem = vec![];
        for i in 0..n { for j in 0..n {
            if i == j { continue; }
            if !games[i][j] { rem.push((i, j)); }
        }}

        let mut br = vec![n; n]; // best possible ranks
        let mut wr = vec![0; n]; // worst possible ranks

        // brute force through possible outcomes
        // 0: left wins, 1: draw, 2: right wins
        for i in 0..3usize.pow(rem.len() as u32) {
            let mut pts = pts.clone();
            let mut i = i;
            for &(a, b) in &rem {
                match i % 3 {
                    0 => pts[a] += 3,
                    1 => { pts[a] += 1; pts[b] += 1; },
                    2 => pts[b] += 3,
                    _ => unreachable!(),
                }
                i /= 3;
            }

            // collect points and define max and min ranks
            let mut p = pts.clone();
            p.sort();
            let mut r = vec![(0, 0); 60];
            let (mut x, mut k) = (p[0], 0);
            for i in 1..n {
                if p[i] != x {
                    r[x as usize] = (k, i-1);
                    k = i; x = p[i];
                }
            }
            r[x as usize] = (k, n-1);

            // update best and worst ranks
            for i in 0..n {
                br[i] = br[i].min(n-r[pts[i] as usize].1);
                wr[i] = wr[i].max(n-r[pts[i] as usize].0);
            }
        }

        // print results
        println!("Group #{} ", tc);
        for i in 0..n {
            println!("{} {}-{}", teams[i], br[i], wr[i]);
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}