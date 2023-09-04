// https://atcoder.jp/contests/abc179/tasks/abc179_d

use proconio::input;
use proconio::fastout;

const MOD: usize = 998_244_353;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut L = Vec::new();
    let mut R = Vec::new();
    for _ in 0..K {
        input! {
            l: usize,
            r: usize,
        }
        L.push(l);
        R.push(r);
    }
    let mut s = vec![0; N];
    let mut DP = vec![0; N];
    s[0] = 1;
    DP[0] = 1;
    for i in 1..N {
        for k in 0..K {
            if i < L[k] {
                continue
            }
            else if i <= R[k] {
                DP[i] += s[i-L[k]];
            }
            else {
                DP[i] += (s[i-L[k]] + MOD - s[i-R[k]-1]) % MOD;
            }
            DP[i] %= MOD;
        }
        s[i] = (s[i-1] + DP[i]) % MOD;
    }
    println!("{}", DP[N-1]);
}