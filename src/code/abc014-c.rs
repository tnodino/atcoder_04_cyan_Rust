// https://atcoder.jp/contests/abc014/tasks/abc014_3

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
    }
    let m = 1_000_000;
    let mut imos = vec![0; m+2];
    for _ in 0..n {
        input! {
            a: usize,
            b: usize,
        }
        imos[a] += 1;
        imos[b+1] -= 1;
    }
    let mut ans = 0;
    for i in 0..=m {
        ans = max(ans, imos[i]);
        imos[i+1] += imos[i];
    }
    println!("{}", ans);
}