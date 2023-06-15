// BOJ 10649 [Guard Mark]
// Supported by GitHub Copilot

use std::io::{self, Read};

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

struct Cow {
    h: i64, w: i64, p: i64
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, h) = (next::<usize>(&mut it), next::<i64>(&mut it));
    let v = (0..n).map(|_| {
        Cow {
            h: next::<i64>(&mut it),
            w: next::<i64>(&mut it),
            p: next::<i64>(&mut it)
        }
    }).collect::<Vec<_>>();

    let mut dp = vec![0; 1<<n];
    let mut hs = vec![0; 1<<n];
    let mut ans = -1;
    for bit in 1..(1<<n) as usize {
        let mut flag = false;
        if bit.next_power_of_two() == bit {
            let i = bit.trailing_zeros() as usize;
            dp[bit] = v[i].p;
            hs[bit] = v[i].h;
            flag = true;
        } else {
            for i in 0..n {
                if bit & (1<<i) != 0 {
                    let pre = bit & !(1<<i);
                    if dp[pre] < v[i].w { continue; }
                    let next = (dp[pre] - v[i].w).min(v[i].p);
                    dp[bit] = dp[bit].max(next);
                    hs[bit] = hs[pre] + v[i].h;
                    flag = true;
                }
            }
        }

        if flag && hs[bit] >= h {
            ans = ans.max(dp[bit]);
        }
    }

    println!("{}", if ans == -1 { "Mark is too tall".to_string() } else { ans.to_string() });
}
