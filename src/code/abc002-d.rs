// https://atcoder.jp/contests/abc002/tasks/abc002_4

use proconio::input;
use std::cmp::max;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut flg = vec![vec![false; N]; N];
    for _ in 0..M {
        input! {
            x: usize,
            y: usize,
        }
        flg[x-1][y-1] = true;
    }
    let mut ans = 0;
    'outer: for bit in 1..1<<N {
        let mut vec = Vec::new();
        for i in 0..N {
            if bit & (1 << i) > 0 {
                vec.push(i);
            }
        }
        for i in 0..vec.len() {
            for j in i+1..vec.len() {
                if !flg[vec[i]][vec[j]] {
                    continue 'outer;
                }
            }
        }
        ans = max(ans, vec.len());
    }
    println!("{}", ans);
}