// BOJ 2630 [Cutting a Colored Paper]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn cut(paper: &Vec<Vec<bool>>) -> (i32, i32) {
    let l = paper.len();
    if l == 1 {
        return match paper[0][0] {
            true => (0, 1),
            false => (1, 0),
        }
    }

    let mut ret = (0, 0);
    let scopes = [
        (0, l / 2, 0, l / 2),
        (0, l / 2, l / 2, l),
        (l / 2, l, 0, l / 2),
        (l / 2, l, l / 2, l),
    ];

    for scope in scopes {
        let mut piece = Vec::new();
        for i in scope.0..scope.1 {
            piece.push(paper[i][scope.2..scope.3].to_vec());
        }
        let res = cut(&piece);
        ret = (ret.0 + res.0, ret.1 + res.1);
    }

    return if ret.0 == 4 && ret.1 == 0 {
        (1, 0)
    } else if ret.0 == 0 && ret.1 == 4 {
        (0, 1)
    } else {
        ret
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let n = it.by_ref().next().unwrap().parse().unwrap();

    let mut paper: Vec<Vec<bool>> = Vec::new();
    for _ in 0..n {
        paper.push(
            it.by_ref().take(n).map(|x| x.parse::<u8>().unwrap() == 1).collect()
        );
    }

    let res = cut(&paper);
    writeln!(so, "{}\n{}", res.0, res.1).unwrap();
}
