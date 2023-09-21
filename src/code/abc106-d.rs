// https://atcoder.jp/contests/abc106/tasks/abc106_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M, Q): (usize, usize, usize),
    }
    let mut sum = vec![vec![0; N+1]; N+1];
    for _ in 0..M {
        input! {
            (L, R): (usize, usize),
        }
        sum[L][R] += 1;
    }
    for i in 1..=N {
        for j in 1..=N {
            sum[i][j] += sum[i-1][j] + sum[i][j-1] - sum[i-1][j-1];
        }
    }
    for _ in 0..Q {
        input! {
            (p, q): (usize, usize),
        }
        println!("{}", sum[q][q] + sum[p-1][p-1] - sum[p-1][q] - sum[q][p-1]);
    }
}