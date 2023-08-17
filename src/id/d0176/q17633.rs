// BOJ 17633 [Sum of Squares]
// Supported by GitHub Copilot

fn mul(a: u64, b: u64, m: u64) -> u64 {
    (a as u128 * b as u128 % m as u128) as u64
}

fn pow(mut a: u64, mut b: u64, m: u64) -> u64 {
    let mut ret = 1;
    while b > 0 {
        if b & 1 > 0 { ret = mul(ret, a, m); }
        a = mul(a, a, m);
        b >>= 1;
    }
    ret
}

fn miller_rabin(n: u64, a: u64) -> bool {
    if n == a { return true }
    let mut d = n - 1;
    while d & 1 == 0 {
        if pow(a, d, n) == n - 1 { return true }
        d >>= 1;
    }
    let tmp = pow(a, d, n);
    return tmp == n - 1 || tmp == 1;
}

const TP: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
fn is_prime(n: u64) -> bool {
    if n < 2 { return false }
    for &p in &TP {
        if n == p { return true }
        if !miller_rabin(n, p) { return false }
    }
    true
}

const SEED: u64 = 14839709780919905126;
const XORA: u64 = 11618625973841399673;
const XORC: u64 = 12090038175212513926;
struct Random {
    state: u64,
}
impl Random {
    fn new() -> Self {
        Random { state: SEED }
    }
    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(XORA).wrapping_add(XORC);
        self.state
    }
    fn next_interval(&mut self, l: u64, r: u64) -> u64 {
        l + self.next() % (r - l)
    }
}

static mut FACTORS: Vec<u64> = Vec::new();
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}
fn pollard_rho(n: u64, rng: &mut Random) {
    if n == 1 { return; }
    for &p in &TP {
        if n % p == 0 {
            unsafe { FACTORS.push(p); }
            pollard_rho(n / p, rng);
            return;
        }
    }
    if is_prime(n) { unsafe { FACTORS.push(n); } return; }
    let mut x = rng.next_interval(2, n);
    let mut y = x;
    let c = rng.next_interval(2, n);
    let mut d = 1;
    while d == 1 {
        x = (mul(x, x, n) + c) % n;
        y = (mul(y, y, n) + c) % n;
        y = (mul(y, y, n) + c) % n;
        let abs = if x > y { x - y } else { y - x };
        d = gcd(abs, n);
    }
    pollard_rho(d, rng);
    pollard_rho(n / d, rng);
}

fn count(v: &[u64]) -> Vec<(u64, u8)> {
    let mut ret = Vec::new();
    let mut cnt = 1;
    for i in 1..v.len() {
        if v[i] == v[i - 1] { cnt += 1; }
        else {
            ret.push((v[i - 1], cnt));
            cnt = 1;
        }
    }
    ret.push((v[v.len() - 1], cnt));
    ret
}
pub fn main() {
    let n = std::io::stdin().lines().next().unwrap().unwrap().parse::<u64>().unwrap();
    if n == ((n as f64).sqrt().floor() as u64).pow(2) {
        println!("1");
        return;
    }

    let mut rng = Random::new();
    pollard_rho(n, &mut rng);
    let mut factors = unsafe { FACTORS.clone() };
    factors.sort_unstable();
    let factors = count(&factors);

    let mut flag = true;
    for (f, c) in factors {
        if f % 4 == 3 && c % 2 != 0 {
            flag = false;
            break;
        }
    }
    if flag { println!("2"); return; }

    let mut n = n;
    while n % 4 == 0 { n /= 4; }
    if n % 8 == 7 { println!("4"); }
    else { println!("3"); }
}
