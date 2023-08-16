// https://atcoder.jp/contests/abc280/tasks/abc280_e

use proconio::input;
use proconio::fastout;

const MOD: usize = 998_244_353;

fn bin_power(mut x: usize, mut k: usize) -> usize {
    let mut ret = 1;
    while k > 0 {
        if k & 1 > 0 {
            ret = (ret * x) % MOD;
        }
        x = (x * x) % MOD;
        k >>= 1;
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: usize,
    }
    let mut DP = vec![0; N+1];
    DP[1] = 1;
    let c = P * bin_power(100, MOD-2) % MOD;
    let n = (100 - P) * bin_power(100, MOD-2) % MOD;
    for i in 2..=N {
        DP[i] = (DP[i-2] * c + DP[i-1] * n) + 1;
        DP[i] %= MOD;
    }
    println!("{}", DP[N]);
}