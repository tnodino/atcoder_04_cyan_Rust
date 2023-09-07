// https://atcoder.jp/contests/abc059/tasks/arc072_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }
    let mut ans = 1<<50;
    for m in 0..2 {
        let mut res = 0;
        let mut s = 0;
        for i in 0..n {
            s += a[i];
            if i % 2 == m {
                if s >= 1 {
                    continue;
                }
                res += s.abs() + 1;
                s = 1;
            }
            else {
                if s <= -1 {
                    continue;
                }
                res += s + 1;
                s = -1;
            }
        }
        ans = min(ans, res);
    }
    println!("{}", ans);
}