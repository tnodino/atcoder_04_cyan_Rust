// https://atcoder.jp/contests/abc046/tasks/arc062_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String
    }
    let N = s.len();
    let cnt = s.chars().filter(|&x| x == 'p').count();
    println!("{}", N / 2 - cnt);
}