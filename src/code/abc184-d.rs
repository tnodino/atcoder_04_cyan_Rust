// https://atcoder.jp/contests/abc184/tasks/abc184_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (A, B, C): (usize, usize, usize),
    }
    let N = 100;
    let mut DP = vec![vec![vec![0.; N+1]; N+1]; N+1];
    for i in (0..N).rev() {
        for j in (0..N).rev() {
            for k in (0..N).rev() {
                if i == 0 && j == 0 && k == 0 {
                    break;
                }
                let x = i as f64;
                let y = j as f64;
                let z = k as f64;
                let s = (x + y + z) as f64;
                DP[i][j][k] += (DP[i+1][j][k] * x / s) + (DP[i][j+1][k] * y / s) + (DP[i][j][k+1] * z / s) + 1.;
            }
        }
    }
    println!("{}", DP[A][B][C]);
}