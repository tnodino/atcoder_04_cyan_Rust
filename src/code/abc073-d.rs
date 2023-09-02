// https://atcoder.jp/contests/abc073/tasks/abc073_d

use proconio::input;
use proconio::fastout;
use itertools::Itertools;
use std::cmp::min;

const INF: usize = 1<<60;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        R: usize,
        r: [usize; R],
    }
    let mut cost = vec![vec![INF; N]; N];
    for i in 0..N {
        cost[i][i] = 0;
    }
    for _ in 0..M {
        input! {
            A: usize,
            B: usize,
            C: usize,
        }
        cost[A-1][B-1] = C;
        cost[B-1][A-1] = C;
    }
    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                cost[i][j] = min(cost[i][j], cost[i][k] + cost[k][j]);
            }
        }
    }
    let mut ans = INF;
    for perm in (0..R).permutations(R) {
        let mut res = 0;
        for i in 0..R-1 {
            res += cost[r[perm[i]]-1][r[perm[i+1]]-1];
        }
        ans = min(ans, res);
    }
    println!("{}", ans);
}