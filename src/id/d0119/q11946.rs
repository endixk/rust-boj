// BOJ 11946 [ACM-ICPC]
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

struct Team {
    cnt: usize,
    score: usize,
    id: usize,
    pen: [usize; 15],
}
const INF: usize = 0x3f3f3f3f;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, _) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut teams = (0..n).map(|i| Team { cnt: 0, score: 0, id: i, pen: [0; 15] }).collect::<Vec<_>>();
    for _ in 0..next(&mut it) {
        let (t, i, q, r) = (
            next::<usize>(&mut it),
            next::<usize>(&mut it)-1,
            next::<usize>(&mut it)-1,
            next::<String>(&mut it),
        );
        match r {
            r if r == "AC" && teams[i].pen[q] < INF => {
                teams[i].cnt += 1;
                teams[i].score += t + teams[i].pen[q] * 20;
                teams[i].pen[q] = INF;
            },
            _ => teams[i].pen[q] += 1,
        }
    }

    teams.sort_unstable_by(|a, b| {
        if a.cnt == b.cnt {
            if a.score == b.score {
                a.id.cmp(&b.id)
            } else {
                a.score.cmp(&b.score)
            }
        } else {
            b.cnt.cmp(&a.cnt)
        }
    });
    for t in teams {
        writeln!(so, "{} {} {}", t.id+1, t.cnt, t.score).unwrap();
    }
}
