// https://atcoder.jp/contests/abc180/tasks/abc180_e

use proconio::input;
use proconio::fastout;
use std::cmp::{max, min};

const INF: isize = 1<<50;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut X = Vec::new();
    let mut Y = Vec::new();
    let mut Z = Vec::new();
    for _ in 0..N {
        input! {
            x: isize,
            y: isize,
            z: isize,
        }
        X.push(x);
        Y.push(y);
        Z.push(z);
    }
    let mut cost = vec![vec![INF; N]; N];
    for i in 0..N {
        for j in 0..N {
            if i == j {
                cost[i][j] = 0;
            }
            else {
                cost[i][j] = (X[j] - X[i]).abs() + (Y[j] - Y[i]).abs() + max(0, Z[j] - Z[i]);
            }
        }
    }
    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                cost[i][j] = min(cost[i][j], cost[i][k] + cost[k][j]);
            }
        }
    }
    let mut DP = vec![vec![INF; 1<<N]; N];
    for i in 1..N {
        DP[i][1<<i] = cost[0][i];
    }
    for bit in 0..1<<N {
        for i in 0..N {
            if bit & (1 << i) > 0 {
                for j in 0..N {
                    if bit & (1 << j) == 0 {
                        DP[j][bit|(1<<j)] = min(DP[j][bit|(1<<j)], DP[i][bit] + cost[i][j]);
                    }
                }
            }
        }
    }
    println!("{}", DP[0][(1<<N)-1]);
}