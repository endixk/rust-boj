// BOJ 2873 [LUNAPARK]
// Supported by GitHub Copilot

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

fn rdldr(r: usize, c: usize) -> String {
    let mut ret = String::new();
    let mut flag = true;
    for _ in 1..r {
        if flag {
            for _ in 1..c { ret.push('R'); }
            ret.push('D');
        } else {
            for _ in 1..c { ret.push('L'); }
            ret.push('D');
        }
        flag = !flag;
    }
    for _ in 1..c { ret.push('R'); }
    ret
}
fn drurd(r: usize, c: usize) -> String {
    let mut ret = String::new();
    let mut flag = true;
    for _ in 1..c {
        if flag {
            for _ in 1..r { ret.push('D'); }
            ret.push('R');
        } else {
            for _ in 1..r { ret.push('U'); }
            ret.push('R');
        }
        flag = !flag;
    }
    for _ in 1..r { ret.push('D'); }
    ret
}
fn rdl(r: usize, c: usize) -> String {
    if r == 0 { return String::new(); }
    let mut ret = String::new();
    for _ in 1..c { ret.push('R'); }
    ret.push('D');
    for _ in 1..c { ret.push('L'); }
    for _ in 1..r/2 {
        ret.push('D');
        for _ in 1..c { ret.push('R'); }
        ret.push('D');
        for _ in 1..c { ret.push('L'); }
    }
    ret.push('D');
    ret
}
fn ldr(r: usize, c: usize) -> String {
    if r == 0 { return String::new(); }
    let mut ret = "D".to_string();
    for _ in 1..c { ret.push('L'); }
    ret.push('D');
    for _ in 1..c { ret.push('R'); }
    for _ in 1..r/2 {
        ret.push('D');
        for _ in 1..c { ret.push('L'); }
        ret.push('D');
        for _ in 1..c { ret.push('R'); }
    }
    ret
}
fn dru(c: usize) -> String {
    let mut ret = String::new();
    for _ in 0..c/2 {
        ret += "DRUR";
    }
    ret
}
fn urd(c: usize) -> String {
    let mut ret = String::new();
    for _ in 0..c/2 {
        ret += "RURD";
    }
    ret
}
fn go(r: usize, c: usize, i: usize, j: usize) -> String {
    let mut ret = String::new();
    ret += &rdl(i - i%2, c);
    ret += &dru(j - j%2);
    ret += if i % 2 == 0 { "DR" } else { "RD" };
    ret += &urd(c - (j + 2 - j%2));
    ret += &ldr(r - (i + 2 - i%2), c);
    ret
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (r, c) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let b = (0..r).map(|_|
        (0..c).map(|_|
            next::<i16>(&mut it)
        ).collect::<Vec<_>>()
    ).collect::<Vec<_>>();
    if r % 2 == 1 {
        writeln!(so, "{}", rdldr(r, c)).unwrap();
        return;
    }
    if c % 2 == 1 {
        writeln!(so, "{}", drurd(r, c)).unwrap();
        return;
    }

    let (mut i, mut j, mut v) = (0, 0, 1000);
    for x in 0..r { for y in 0..c {
        if (x + y) % 2 == 1 && b[x][y] < v {
            i = x; j = y; v = b[x][y];
        }
    }}
    writeln!(so, "{}", go(r, c, i, j)).unwrap();
}
