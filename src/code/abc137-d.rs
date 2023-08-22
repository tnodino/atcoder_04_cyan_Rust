// https://atcoder.jp/contests/abc137/tasks/abc137_d

use proconio::input;
use proconio::fastout;
use std::collections::BinaryHeap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            A: usize,
            B: usize,
        }
        vec.push((A, B));
    }
    vec.push((M+1, 0));
    vec.sort();
    let mut bh = BinaryHeap::new();
    let mut ans = 0;
    let mut idx = 0;
    for i in 1..=M {
        while vec[idx].0 == i {
            bh.push(vec[idx].1);
            idx += 1;
        }
        if !bh.is_empty() {
            ans += bh.pop().unwrap();
        }
    }
    println!("{}", ans);
}