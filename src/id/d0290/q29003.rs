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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut front = [[false, false]; 100001];
    let mut back = [[false, false]; 100001];
    let (mut fi, mut bi) = (0, 0);

    let (mut red, mut blue, mut black, mut yellow, mut pink) = (
        vec![], vec![], vec![], vec![], vec![]
    );
    for i in 1..=k {
        let (x, y) = (next::<usize>(&mut it)-1, next::<usize>(&mut it)-1);
        pink.push(i);
        if x == fi {
            if front[fi][1-y] {
                red.push(i);
                blue.push(i);
            } else if y == 0 {
                red.push(i);
            } else {
                blue.push(i);
            }
        }
        if x < 100001 {
            front[x][y] = true;
            while front[fi][0] && front[fi][1] {
                fi += 1;
            }
        }

        if x == n-bi-1 {
            if back[bi][1-y] {
                black.push(i);
                yellow.push(i);
            } else if y == 0 {
                black.push(i);
            } else {
                yellow.push(i);
            }
        }
        if n-x-1 < 100001 {
            back[n-x-1][y] = true;
            while back[bi][0] && back[bi][1] {
                bi += 1;
            }
        }
    }

    write!(so, "{} ", red.len()).unwrap();
    for x in red { write!(so, "{} ", x).unwrap(); }
    writeln!(so).unwrap();
    write!(so, "{} ", blue.len()).unwrap();
    for x in blue { write!(so, "{} ", x).unwrap(); }
    writeln!(so).unwrap();
    write!(so, "{} ", black.len()).unwrap();
    for x in black { write!(so, "{} ", x).unwrap(); }
    writeln!(so).unwrap();
    write!(so, "{} ", yellow.len()).unwrap();
    for x in yellow { write!(so, "{} ", x).unwrap(); }
    writeln!(so).unwrap();
    write!(so, "{} ", pink.len()).unwrap();
    for x in pink { write!(so, "{} ", x).unwrap(); }
    writeln!(so).unwrap();
}
