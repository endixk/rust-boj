// BOJ 25213 [Pieces of Cake (Hard)]
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

const MOD: i64 = 1_000_000_007;
const CUT: usize = 19;
const MAX: usize = 100_001;
fn gcd(a: i64, b: i64) -> i64 { if b == 0 { a } else { gcd(b, a % b) } }
fn lcm(a: i64, b: i64) -> i64 { a / gcd(a, b) * b }
fn ncr(n: usize, r: usize, fmod: &[i64], finv: &[i64]) -> i64 { fmod[n] * finv[r] % MOD * finv[n-r] % MOD }
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut c = vec![0; 26];
    for _ in 0..next(&mut it){
        c[next::<usize>(&mut it)] += 1;
    }

    // precompute
    let mut l = 1;
    for i in 2..26 { l = lcm(l, i); }
    let mut d = vec![1; MAX];
    let mut fmod = vec![1; MAX];
    let mut finv = vec![1; MAX];
    for i in 2..MAX {
        d[i] = -(MOD / i as i64) * d[MOD as usize % i] % MOD;
        fmod[i] = fmod[i-1] * i as i64 % MOD;
        finv[i] = finv[i-1] * d[i] % MOD;
    }

    let mut case = vec![];
    case.push((0, 1));
    for i in 2..CUT {
        let mut m = vec![];
        for j in 1..=c[i] {
            if j > i { break; }
            for &(a, p) in &case {
                let x = a + l / i as i64 * j as i64;
                if 100 * x <= 101 * l {
                    m.push((x, p * ncr(c[i], j, &fmod, &finv) % MOD));
                }
            }
        }
        for (x, p) in m {
            case.push((x, p));
        }
    }
    case.sort_unstable();

    let mut ps = vec![0];
    let mut s = 0;
    for &(_, p) in &case { s = (s + p) % MOD; ps.push(s); }

    let mut esac = vec![];
    esac.push((0, 1));
    for i in CUT..26 {
        let mut m = vec![];
        for j in 1..=c[i] {
            if j > i { break; }
            for &(a, p) in &esac {
                let x = a + l / i as i64 * j as i64;
                if 100 * x <= 101 * l {
                    m.push((x, p * ncr(c[i], j, &fmod, &finv) % MOD));
                }
            }
        }
        for (x, p) in m {
            esac.push((x, p));
        }
    }

    let mut ans = 0;
    for (x, p) in esac {
        let a = case.partition_point(|&(y, _)| 100 * x + 100 * y < 99 * l);
        let b = case.partition_point(|&(y, _)| 100 * x + 100 * y <= 101 * l);
        ans = (ans + p * (ps[b] - ps[a]) % MOD) % MOD;
    }
    writeln!(so, "{}", (ans + MOD) % MOD)?;

    Ok(())
}
