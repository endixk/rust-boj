// BOJ 15652 [N and M (4)]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn track(so: &mut io::BufWriter<io::StdoutLock>, n: usize, v: &mut [u8], k: usize) {
    if k == 0 {
        v.iter().rev().for_each(|x| write!(so, "{} ", x).unwrap());
        writeln!(so).unwrap();
    } else {
        let mut mx = 1;
        for i in 1..=n {
            if k >= v.len() {break;}
            if v[k] >= i as u8 {
                mx = i;
            }
        }
        for i in mx..=n {
            v[k - 1] = i as u8;
            track(so, n, v, k - 1);
        }
    }
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let v = read(&mut si).split_ascii_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
    track(&mut so, v[0], &mut vec![0; v[1]], v[1]);
}
