// BOJ 1230 [String Distance]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let v = s.split('\n').collect::<Vec<&str>>();

    const INF: u16 = 55555;
    let (lx, ly) = (v[0].len()+1, v[1].len()+1);

    let mut mat = [INF; 1111111]; // match dp
    let mut ins = [INF; 1111111]; // insert dp

    mat[0] = 0;
    for y in 1..ly { ins[y] = 1; }
    for (i, a) in v[0].chars().enumerate() {
        for (j, b) in v[1].chars().enumerate() {
            if a == b {
                mat[(i+1)*ly+j+1] = mat[i*ly+j].min(ins[i*ly+j]);
            }
            ins[(i+1)*ly+j+1] = ins[(i+1)*ly+j].min(mat[(i+1)*ly+j]+1);
        }
    }

    let ans = mat[lx*ly-1].min(ins[lx*ly-1]);
    writeln!(so, "{}", match ans { INF => "-1".to_string(), _ => ans.to_string() }).unwrap();
}
