// https://atcoder.jp/contests/abc270/tasks/abc270_d

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; K],
    }
    let mut DP = vec![0; N+1];
    for i in 1..=N {
        let mut ma = 0;
        for j in 0..K {
            if A[j] <= i {
                ma = max(ma, A[j] + (i - A[j]) - DP[i-A[j]]);
            }
        }
        DP[i] = ma;
    }
    println!("{}", DP[N]);
}