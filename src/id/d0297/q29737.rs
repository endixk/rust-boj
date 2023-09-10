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

#[derive(Default, Clone)] struct Stat {
    name: String,
    streak: i32,
    freeze: i32,
    begin: i32,
    fail: i32,
}
impl Stat {
    fn analyze(&mut self, info: String) {
        let (mut ms, mut mr, mut mb, mut f) = (0, 0, 0, 0);
        let (mut s, mut r, mut b) = (0, 0, 0);
        let ib = info.as_bytes();
        for (i, c) in info.chars().enumerate() {
            match c {
                'O' => {
                    if s == 0 { b = i as i32; }
                    s += 1;
                },
                'F' => {
                    if s > 0 { r += 1; }
                },
                'X' => {
                    f += 1;
                    if s > 0 {
                        let mut j = i-1;
                        while ib[j] == b'F' {
                            r -= 1;
                            j -= 1;
                        }
                        if s > ms { ms = s; mr = r; mb = b; }
                        else if s == ms && r < mr { mr = r; mb = b; }
                    }
                    s = 0; r = 0;
                },
                _ => unreachable!()
            }
        }
        if s > 0 {
            let mut j = ib.len()-1;
            while ib[j] == b'F' {
                r -= 1;
                j -= 1;
            }
            if s > ms { ms = s; mr = r; mb = b; }
            else if s == ms && r < mr { mr = r; mb = b; }
        }

        self.streak = ms;
        self.freeze = mr;
        self.begin = mb;
        self.fail = f;
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, w) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut stats = vec![Stat::default(); n];
    for i in 0..n {
        stats[i].name = next::<String>(&mut it);
        let mut log = vec![' '; 7*w];
        for h in 0..7 {
            let s = next::<String>(&mut it);
            for (i, c) in s.chars().enumerate() {
                log[i*7+h] = c;
            }
        }
        stats[i].analyze(log.into_iter().collect());
    }

    stats.sort_by(|a, b| {
        if a.streak != b.streak { b.streak.cmp(&a.streak) }
        else if a.freeze != b.freeze { a.freeze.cmp(&b.freeze) }
        else if a.begin != b.begin { a.begin.cmp(&b.begin) }
        else if a.fail != b.fail { a.fail.cmp(&b.fail) }
        else { a.name.cmp(&b.name) }
    });
    let mut rank = 1;
    writeln!(so, "{}. {}", rank, stats[0].name).unwrap();
    for i in 1..n {
        if stats[i].streak != stats[i-1].streak ||
            stats[i].freeze != stats[i-1].freeze ||
            stats[i].begin != stats[i-1].begin ||
            stats[i].fail != stats[i-1].fail {
            rank += 1;
        }
        writeln!(so, "{}. {}", rank, stats[i].name).unwrap();
    }
}