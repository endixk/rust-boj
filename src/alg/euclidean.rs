fn gcd(a: i64, b: i64) -> i64 { if b == 0 { a } else { gcd(b, a % b) } }
fn lcm(a: i64, b: i64) -> i64 { a / gcd(a, b) * b }

fn xgcd(a: i64, b: i64) -> (i64, i64, i64) {
    return if b == 0 { (a, 1, 0) }
    else {
        let (g, x, y) = xgcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}
fn mmi(a: i64, m: i64) -> i64 {
    let (_, x, _) = xgcd(a, m);
    (x + m) % m
}
fn crt(a: i64, b: i64, m: i64, n: i64) -> i64 {
    let (_, x, y) = xgcd(m, n);
    let k = m * n;
    (a * n % k * y % k + b * m % k * x % k + k) % k
}

const MOD: i64 = 1_000_000_007;
const MAX: usize = 100_001;
static mut GEN: bool = false;
static mut FMOD: [i64; MAX] = [1; MAX];
static mut FINV: [i64; MAX] = [1; MAX];
unsafe fn gen() {
    let mut dp = vec![1; MAX];
    for i in 2..MAX {
        dp[i] = -(MOD / i as i64) * dp[MOD as usize % i] % MOD;
        FMOD[i] = FMOD[i-1] * i as i64 % MOD;
        FINV[i] = FINV[i-1] * dp[i] % MOD;
    }
}
fn ncr(n: usize, r: usize) -> i64 { unsafe { if !GEN { gen(); GEN = true } FMOD[n] * FINV[r] % MOD * FINV[n-r] % MOD  } }