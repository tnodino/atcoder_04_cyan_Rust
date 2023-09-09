// https://atcoder.jp/contests/abc186/tasks/abc186_e

use proconio::input;
use proconio::fastout;
use ac_library::ModInt as Mint;
use num::integer::gcd;

#[allow(non_snake_case)]
fn solve() {
    input! {
        (mut N, mut S, mut K): (usize, usize, usize),
    }
    let g = gcd(N, gcd(S, K));
    N /= g;
    S /= g;
    K /= g;
    if gcd(N, K) > 1 {
        println!("-1");
        return;
    }
    Mint::set_modulus(N as u32);
    let S = Mint::new(S);
    let K = Mint::new(K);
    println!("{}", -S / K);
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
    }
    for _ in 0..T {
        solve();
    }
}