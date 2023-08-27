// https://atcoder.jp/contests/abc032/tasks/abc032_c

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        s: [usize; N],
    }
    let mut l = 0;
    let mut x = 1;
    let mut ans = 0;
    for r in 0..N {
        x *= s[r];
        while l <= r && x > K {
            x /= s[l];
            l += 1;
        }
        ans = max(ans, r + 1 - l);
    }
    if x == 0 {
        println!("{}", N);
    }
    else {
        println!("{}", ans);
    }
}