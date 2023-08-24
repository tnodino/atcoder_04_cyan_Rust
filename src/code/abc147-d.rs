// https://atcoder.jp/contests/abc147/tasks/abc147_d

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let M = 60;
    let mut cnt = vec![vec![0, 0]; M];
    for i in 0..N {
        for j in 0..M {
            if A[i] & (1 << j) == 0 {
                cnt[j][0] += 1;
            }
            else {
                cnt[j][1] += 1;
            }
        }
    }
    let mut ans = 0;
    for i in 0..M {
        ans += (1 << i) % MOD * cnt[i][0] % MOD * cnt[i][1] % MOD;
        ans %= MOD;
    }
    println!("{}", ans);
}