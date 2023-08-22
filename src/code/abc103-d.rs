// https://atcoder.jp/contests/abc103/tasks/abc103_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        M: usize,
    }
    let mut vec = Vec::new();
    for _ in 0..M {
        input! {
            a: usize,
            b: usize,
        }
        vec.push((a, b));
    }
    vec.sort_by(|a, b|
    if a.1 != b.1 {
        a.1.cmp(&b.1)
    }
    else {
        a.0.cmp(&b.0)
    });
    let mut l = 0;
    let mut ans = 0;
    for i in 0..M {
        if l < vec[i].0 {
            l = vec[i].1 - 1;
            ans += 1;
        }
    }
    println!("{}", ans);
}