// BOJ 26586 [Feed Store]
// Supported by GitHub Copilot

const INF: i32 = 0x3f3f3f3f;
fn cost(bit: usize, f: &[i32], cnt: usize) -> i32 {
    let mut ret = 0;
    for i in 0..cnt {
        if bit & (1 << i) != 0 {
            ret += f[i];
        }
    }
    ret
}
fn go(dp: &mut Vec<Vec<i32>>, tr: &mut Vec<Vec<usize>>, bit: usize, cur: usize, adj: &Vec<Vec<(usize,i32)>>, cst: &[i32], x: i32) -> i32 {
    if dp[bit][cur] != -1 { return dp[bit][cur]; }
    if cst[bit] >= x && cur == 0 { dp[bit][cur] = 0; return 0; }
    dp[bit][cur] = INF;
    let (mut min, mut idx) = (INF, 0);
    for &(nxt, w) in adj[cur].iter() {
        let x = go(dp, tr, bit | (1 << nxt), nxt, adj, cst, x) + w;
        if x < min { min = x; idx = nxt; }
    }
    dp[bit][cur] = min;
    tr[bit][cur] = idx;
    dp[bit][cur]
}
pub fn main() { read();
    let x = next::<i32>();
    let v = unsafe { IT.as_mut().unwrap().map(|s| s.to_string()).collect::<Vec<_>>() };
    let mut f = vec![0; 26];
    let mut adj = vec![vec![]; 26];
    let mut map = vec![0; 26];
    let mut rmp = vec![' '; 26];
    let mut cnt = 1;
    rmp[0] = 'A';
    for s in &v {
        if s.len() == 2 {
            let (i, j) = (s.as_bytes()[0] - b'A', s.as_bytes()[1] - b'0');
            let (i, j) = (i as usize, j as i32);
            map[i] = cnt; f[cnt] = j; rmp[cnt] = s.as_bytes()[0] as char; cnt += 1;
        } else {
            let (u, v) = (s.as_bytes()[0] - b'A', s.as_bytes()[2] - b'A');
            let (u, v) = (map[u as usize], map[v as usize]);
            let w = s[4..].parse::<i32>().unwrap();
            adj[u].push((v, w));
            adj[v].push((u, w));
        }
    }

    let mut dp = vec![vec![-1; cnt]; 1<<cnt];
    let mut tr = vec![vec![0; cnt]; 1<<cnt];
    let mut cst = vec![0; 1<<cnt];
    for bit in 0..1<<cnt {
        cst[bit] = cost(bit, &f, cnt);
    }
    let ans = go(&mut dp, &mut tr, 0, 0, &adj, &cst, x);

    let mut bit = 0;
    let mut cur = 0;
    let mut path = vec!['A'];
    while dp[bit][cur] != 0 {
        let nxt = tr[bit][cur];
        path.push(rmp[nxt]);
        bit |= 1 << nxt;
        cur = nxt;
    }

    println!("({}) - {}", ans, path.iter().collect::<String>());
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