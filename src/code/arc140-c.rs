// https://atcoder.jp/contests/arc140/tasks/arc140_c

use proconio::input;
use proconio::fastout;
use std::mem::swap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
    }
    let M;
    if X <= N / 2 {
        M = N / 2 + 1;
    }
    else {
        M = N / 2;
    }
    let mut L = Vec::new();
    let mut R = Vec::new();
    for i in (1..=M).rev() {
        if X != i {
            L.push(i);
        }
    }
    for i in M+1..=N {
        if X != i {
            R.push(i);
        }
    }
    if N % 2 == 0 && X <= N / 2 {
        for i in 0..R.len() {
            swap(&mut L[i+1], &mut R[i]);
        }
    }
    let mut vec = vec![X];
    for i in 0..R.len() {
        vec.push(L[i]);
        vec.push(R[i]);
    }
    if L.len() > R.len() {
        vec.push(*L.last().unwrap());
    }
    println!("{}", vec.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}