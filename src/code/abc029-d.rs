// https://atcoder.jp/contests/abc029/tasks/abc029_d

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
    }
    let mut ans = 0;
    for i in 0..10 {
        let L = N / pow(10, i+1);
        let R = N % pow(10, i);
        let M = (N % pow(10, i+1) - N % pow(10, i)) / pow(10, i);
        ans += L * pow(10, i);
        if M >= 2 {
            ans += pow(10, i);
        }
        else if M >= 1 {
            ans += R + 1;
        }
    }
    println!("{}", ans);
}