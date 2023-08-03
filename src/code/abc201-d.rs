// https://atcoder.jp/contests/abc201/tasks/abc201_d

use proconio::input;
use proconio::fastout;
use std::cmp::{min, max};

const INF: isize = 1<<32;

fn f(a: char) -> isize {
    if a == '+' {
        return 1;
    }
    return -1;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    let mut A = Vec::new();
    for _ in 0..H {
        input! {
            a: String,
        }
        let a = a.chars().collect::<Vec<char>>();
        A.push(a);
    }
    let mut DP = vec![vec![0; W]; H];
    for i in 0..H {
        for j in 0..W {
            if (i + j) % 2 == 0 {
                DP[i][j] = -INF;
            }
            else {
                DP[i][j] = INF;
            }
        }
    }
    DP[H-1][W-1] = 0;
    for i in (0..H).rev() {
        for j in (0..W).rev() {
            if (i + j) % 2 == 0 {
                if i + 1 < H {
                    DP[i][j] = max(DP[i][j], DP[i+1][j] + f(A[i+1][j]));
                }
                if j + 1 < W {
                    DP[i][j] = max(DP[i][j], DP[i][j+1] + f(A[i][j+1]));
                }
            }
            else {
                if i + 1 < H {
                    DP[i][j] = min(DP[i][j], DP[i+1][j] - f(A[i+1][j]));
                }
                if j + 1 < W {
                    DP[i][j] = min(DP[i][j], DP[i][j+1] - f(A[i][j+1]));
                }
            }
        }
    }
    if DP[0][0] > 0 {
        println!("Takahashi");
    }
    else if DP[0][0] < 0 {
        println!("Aoki");
    }
    else {
        println!("Draw");
    }
}