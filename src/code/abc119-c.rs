// https://atcoder.jp/contests/abc119/tasks/abc119_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, A, B, C): (usize, isize, isize, isize),
        l: [isize; N],
    }
    let M: usize = pow(4, N);
    let mut ans = 1<<50;
    for bit in 0..M {
        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut c = Vec::new();
        let mut x = bit;
        for i in 0..N {
            match x % 4 {
                0 => a.push(l[i]),
                1 => b.push(l[i]),
                2 => c.push(l[i]),
                _ => {},
            }
            x /= 4;
        }
        if a.is_empty() || b.is_empty() || c.is_empty() {
            continue;
        }
        let res = (a.len() as isize - 1) * 10
            + (b.len() as isize - 1) * 10
            + (c.len() as isize - 1) * 10
            + (A - a.iter().sum::<isize>()).abs()
            + (B - b.iter().sum::<isize>()).abs()
            + (C - c.iter().sum::<isize>()).abs();
        ans = min(ans, res);
    }
    println!("{}", ans);
}