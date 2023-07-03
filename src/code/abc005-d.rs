// https://atcoder.jp/contests/abc005/tasks/abc005_4

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        D: [[usize; N]; N],
        Q: usize,
    }
    let mut S = vec![vec![0; N+1]; N+1];
    for i in 1..=N {
        for j in 1..=N {
            S[i][j] = D[i-1][j-1] + S[i-1][j] + S[i][j-1] - S[i-1][j-1];
        }
    }
    let M = N * N;
    let mut cnt = vec![0; M+1];
    for a in 1..=N {
        for b in 1..=N {
            for c in a..=N {
                for d in b..=N {
                    let x = (c - a + 1) * (d - b + 1);
                    cnt[x] = max(cnt[x], S[c][d] + S[a-1][b-1] - S[a-1][d] - S[c][b-1]);
                }
            }
        }
    }
    for i in 1..=M {
        cnt[i] = max(cnt[i-1], cnt[i]);
    }
    for _ in 0..Q {
        input! {
            P: usize,
        }
        println!("{}", cnt[P]);
    }
}