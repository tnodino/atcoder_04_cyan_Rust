// https://atcoder.jp/contests/abc142/tasks/abc142_e

use proconio::input;
use proconio::fastout;
use std::cmp::min;

const INF: usize = 1<<60;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut DP = vec![vec![INF; 1<<N]; M+1];
    DP[0][0] = 0;
    for i in 0..M {
        input! {
            a: usize,
            b: usize,
            c: [usize; b],
        }
        let mut cbit = 0;
        for j in 0..b {
            cbit |= 1 << (c[j] - 1);
        }
        for bit in 0..1<<N {
            DP[i+1][bit] = min(DP[i+1][bit], DP[i][bit]);
            DP[i+1][bit|cbit] = min(DP[i+1][bit|cbit], DP[i][bit] + a);
        }
    }
    if DP[M][(1<<N)-1] == INF {
        println!("-1");
    }
    else {
        println!("{}", DP[M][(1<<N)-1]);
    }
}