// https://atcoder.jp/contests/abc105/tasks/abc105_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: isize,
    }
    if N == 0 {
        println!("0");
        return;
    }
    let mut ans = Vec::new();
    while N != 0 {
        let bit = N.abs() % 2;
        ans.push(bit);
        N = (N - bit) / -2;
    }
    println!("{}", ans.iter().rev().map(|&x| x.to_string()).collect::<String>());
}