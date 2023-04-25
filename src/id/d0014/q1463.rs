// BOJ 1463 [Down to One]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let n = read(&mut si).parse::<usize>().unwrap();
    let mut memo = vec![0; n + 1];

    for i in 2..=n {
        memo[i] = memo[i - 1] + 1;
        if i % 2 == 0 {
            memo[i] = memo[i].min(memo[i / 2] + 1);
        }
        if i % 3 == 0 {
            memo[i] = memo[i].min(memo[i / 3] + 1);
        }
    }

    writeln!(so, "{}", memo[n]).unwrap();
}
