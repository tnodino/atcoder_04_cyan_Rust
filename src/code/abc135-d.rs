// https://atcoder.jp/contests/abc135/tasks/abc135_d

use proconio::input;
use proconio::fastout;
use ac_library::ModInt1000000007 as Mint;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    let M = 13;
    let K = 10;
    let S = S.chars().collect::<Vec<char>>();
    let mut DP = vec![vec![Mint::new(0); M]; N+1];
    DP[0][0] = 1.into();
    for i in 0..N {
        for j in 0..M {
            for k in 0..K {
                if S[i] == '?' || (S[i] as usize) - ('0' as usize) == k {
                    let idx = (j * 10 + k) % M;
                    DP[i+1][idx] = DP[i+1][idx] + DP[i][j];
                }
            }
        }
    }
    println!("{}", DP[N][5]);
}