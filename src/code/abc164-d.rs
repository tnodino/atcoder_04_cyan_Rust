// https://atcoder.jp/contests/abc164/tasks/abc164_d

use proconio::input;
use proconio::fastout;

const MOD: usize = 2019;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    let S = S.chars().rev().collect::<Vec<char>>();
    let mut x = 0;
    let mut p = 1;
    let mut cnt = vec![0; MOD];
    cnt[0] = 1;
    let mut ans: usize = 0;
    for i in 0..N {
        x += ((S[i] as usize) - ('0' as usize)) * p;
        x %= MOD;
        p *= 10;
        p %= MOD;
        ans += cnt[x];
        cnt[x] += 1;
    }
    println!("{}", ans);
}