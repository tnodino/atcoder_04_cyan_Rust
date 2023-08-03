// https://atcoder.jp/contests/abc200/tasks/abc200_d

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let M = 200;
    let K = min(N, 10);
    let mut vec: Vec<Vec<usize>> = vec![Vec::new(); M];
    for bit in 1..1<<K {
        let mut res = Vec::new();
        let mut idx = 0;
        for i in 0..K {
            if bit & (1 << i) > 0 {
                res.push(i+1);
                idx += A[i];
                idx %= M;
            }
        }
        if !vec[idx].is_empty() {
            println!("Yes");
            println!("{} {}", vec[idx].len(), vec[idx].iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
            println!("{} {}", res.len(), res.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
            return;
        }
        else {
            vec[idx] = res;
        }
    }
    println!("No");
}