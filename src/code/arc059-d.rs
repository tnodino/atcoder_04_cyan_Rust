// https://atcoder.jp/contests/arc059/tasks/arc059_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String,
    }
    let N = s.len();
    let s = s.chars().collect::<Vec<char>>();
    for i in 0..N-1 {
        if s[i] == s[i+1] {
            println!("{} {}", i+1, i+2);
            return;
        }
    }
    for i in 0..N-2 {
        if s[i] == s[i+2] {
            println!("{} {}", i+1, i+3);
            return;
        }
    }
    println!("-1 -1");
}