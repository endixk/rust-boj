use std::io::{self, Read, Write};
fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}
fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

const MOD: i32 = 1_000_000_007;
// 0: *, 1: ~I, 2: I, 3: A
fn row_valid(stat: &[usize], m: usize) -> bool {
    for i in 0..m {
        if stat[i] == 1 {
            if i > 0 && stat[i-1] > 0 { return false; }
            if i < m-1 && stat[i+1] > 0 { return false; }
        } else if stat[i] == 2 {
            if i > 0 && (stat[i-1] == 1 || stat[i-1] == 3) { return false; }
            if i < m-1 && (stat[i+1] == 1 || stat[i+1] == 3) { return false; }
        } else if stat[i] == 3 {
            if i > 0 && (stat[i-1] == 1 || stat[i-1] == 2) { return false; }
            if i < m-1 && (stat[i+1] == 1 || stat[i+1] == 2) { return false; }
        }
    }
    true
}
fn valid(stat: &[usize], pstat: &[usize], x: usize, m: usize, room: &Vec<Vec<usize>>) -> bool {
    for i in 0..m {
        if pstat[i] == 0 {
            if room[x-1][i] == 3 { return false; }
            if stat[i] == 1 { return false; }
        } else if pstat[i] == 1 {
            if room[x-1][i] == 3 { return false; }
            if i > 0 && pstat[i-1] > 0 { return false; }
            if i < m-1 && pstat[i+1] > 0 { return false; }
            if stat[i] > 0 { return false; }
        } else if pstat[i] == 2 {
            if room[x-1][i] == 3 { return false; }
            if stat[i] == 3 { return false; }
            let mut flag = false;
            if i > 0 { flag |= pstat[i-1] == 2; }
            if i < m-1 { flag |= pstat[i+1] == 2; }
            flag |= stat[i] == 1 || stat[i] == 2;
            if !flag { return false; }
        } else if pstat[i] == 3 {
            if room[x-1][i] == 0 { return false; }
            if i > 0 && pstat[i-1] == 1 { return false; }
            if i > 0 && pstat[i-1] == 2 { return false; }
            if i < m-1 && pstat[i+1] == 1 { return false; }
            if i < m-1 && pstat[i+1] == 2 { return false; }
            if stat[i] == 1 { return false; }
            if stat[i] == 2 { return false; }
        }
    }
    true
}
fn go(dp: &mut Vec<Vec<Vec<i32>>>, i: usize, b: usize, k: usize, m: usize, room: &Vec<Vec<usize>>,
      vbits: &[usize], stats: &Vec<Vec<usize>>, vstat: &[bool], cstat: &[usize]) -> i32 {
    if i == 0 {
        if k > 0 { return 0; }
        if !vstat[b] { return 0; }
        return 1;
    }
    if dp[i][b][k] != -1 { return dp[i][b][k]; }
    let mut ret = 0;
    let stat = &stats[b];
    for p in 0..vbits.len() {
        let pstat = &stats[p];
        let cnt = cstat[p];
        if k < cnt { continue; }
        if !valid(&stat, &pstat, i, m, room) { continue; }
        ret = (ret + go(dp, i-1, p, k-cnt, m, room, vbits, stats, vstat, cstat)) % MOD;
    }
    dp[i][b][k] = ret;
    ret
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m, x, _) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut room = Vec::new();
    for _ in 0..n {
        let s = next::<String>(&mut it);
        room.push(s.chars().map(|c| match c { 'A' => 3usize, _ => 0usize }).collect::<Vec<_>>());
    }
    room.push(vec![0usize; m]);

    let mut vbits = vec![];
    for bit in 0..4usize.pow(m as u32) {
        let stat = (0..m).map(|j| (bit / 4usize.pow(j as u32)) % 4).collect::<Vec<_>>();
        if row_valid(&stat, m) {
            vbits.push(bit);
        }
    }

    let mut vstat = vec![true; vbits.len()];
    let mut cstat = vec![0; vbits.len()];
    // let mut astat = vec![false; vbits.len()];
    let mut stats = vec![];
    for (i, &bit) in vbits.iter().enumerate() {
        let stat = (0..m).map(|j| (bit / 4usize.pow(j as u32)) % 4).collect::<Vec<_>>();
        vstat[i] = stat.iter().all(|&x| x != 1);
        cstat[i] = stat.iter().filter(|&&x| x == 1 || x == 2).count();
        // astat[i] = stat.iter().any(|&x| x == 3);
        stats.push(stat);
    }

    let mut dp = vec![vec![vec![-1; x+1]; vbits.len()]; n+1];
    writeln!(so, "{}", go(&mut dp, n, 0, x, m, &room, &vbits, &stats, &vstat, &cstat)).unwrap();
}