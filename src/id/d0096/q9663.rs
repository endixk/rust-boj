// BOJ 9663 [N-Queen]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn possible(cols: &Vec<i8>, col: i8) -> bool {
    let row = cols.len();
    for i in 0..row {
        if col == cols[i] || (row - i) as i8 == (col - cols[i]).abs() {
            return false;
        }
    }
    true
}

fn solve(cols: &mut Vec<i8>, cap: i8) -> u32 {
    let row = cols.len() as i8;
    if row == cap {
        return 1;
    }

    let mut ret = 0;
    for col in 0..cap {
        if possible(cols, col) {
            cols.push(col);
            ret += solve(cols, cap);
            cols.pop();
        }
    }
    ret
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let n: i8 = read(&mut si).trim().parse().unwrap();
    let mut cols = Vec::new();
    writeln!(so, "{}", solve(&mut cols, n)).unwrap();
}
