// https://atcoder.jp/contests/abc128/tasks/abc128_d

use proconio::input;
use proconio::fastout;
use std::cmp::{max, min};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
        V: [isize; N],
    }
    let mut ans = 0;
    for l in 0..=N {
        for r in 0..=N {
            if l + r > N || l + r > K {
                break;
            }
            let mut res = 0;
            let mut vec = Vec::new();
            for i in 0..l {
                res += V[i];
                vec.push(V[i]);
            }
            for i in 0..r {
                res += V[N-i-1];
                vec.push(V[N-i-1]);
            }
            vec.sort();
            let k = min(vec.len(), K - (l + r));
            for i in 0..k {
                if vec[i] >= 0 {
                    break;
                }
                res -= vec[i];
            }
            ans = max(ans, res);
        }
    }
    println!("{}", ans);
}