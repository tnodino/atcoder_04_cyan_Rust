// https://atcoder.jp/contests/abc096/tasks/abc096_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = 55_555;
    let mut sieve = vec![true; M+1];
    let mut ans = Vec::new();
    for i in 2..=M {
        if sieve[i] {
            for j in (i..=M).step_by(i) {
                sieve[j] = false;
            }
            if i % 5 == 1 {
                ans.push(i);
            }
        }
        if ans.len() == N {
            break;
        }
    }
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}