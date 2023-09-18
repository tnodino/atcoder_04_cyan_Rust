// https://atcoder.jp/contests/abc164/tasks/abc164_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    let M = 2019;
    let S = S.chars().map(|x| (x as usize) - ('0' as usize)).collect::<Vec<usize>>();
    let mut cnt = vec![0; M];
    cnt[0] += 1;
    let mut p = 1;
    let mut s = 0;
    let mut ans = 0;
    for i in (0..N).rev() {
        s += S[i] * p;
        s %= M;
        p *= 10;
        p %= M;
        ans += cnt[s];
        cnt[s] += 1;
    }
    println!("{}", ans);
}