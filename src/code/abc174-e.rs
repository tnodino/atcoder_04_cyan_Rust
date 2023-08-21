// https://atcoder.jp/contests/abc174/tasks/abc174_e

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn f(x: usize, N: usize, K: usize, A: &Vec<usize>) -> bool {
    let mut cnt = 0;
    for i in 0..N {
        cnt += (A[i] + x - 1) / x - 1;
    }
    if cnt <= K {
        return true;
    }
    return false;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N],
    }
    let mut ng = 1;
    let mut ok = 1_000_000_000;
    for _ in 0..100 {
        let mid = (ok + ng) / 2;
        if f(mid, N, K, &A) {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    println!("{}", ok);
}