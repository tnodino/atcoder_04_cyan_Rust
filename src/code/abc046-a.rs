// https://atcoder.jp/contests/abc046/tasks/arc062_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut x = 1;
    let mut y = 1;
    for _ in 0..N {
        input! {
            T: usize,
            A: usize,
        }
        let m = max((x + T - 1) / T, (y + A - 1) / A);
        x = T * m;
        y = A * m;
    }
    println!("{}", x + y);
}