// https://atcoder.jp/contests/abc044/tasks/arc060_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
        x: [usize; N],
    }
    let M = N * 50;
    let mut DP = vec![vec![vec![0; M+1]; N+1]; N+1];
    DP[0][0][0] = 1;
    for i in 0..N {
        for j in 0..N {
            for k in 0..=M {
                DP[i+1][j][k] += DP[i][j][k];
                if x[i] + k <= M {
                    DP[i+1][j+1][x[i]+k] += DP[i][j][k];
                }
            }
        }
    }
    let mut ans: usize = 0;
    for i in 1..=N {
        ans += DP[N][i][A*i];
    }
    println!("{}", ans);
}