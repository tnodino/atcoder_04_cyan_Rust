// https://atcoder.jp/contests/abc100/tasks/abc100_d

use proconio::input;
use proconio::fastout;
use std::cmp::max;

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
            x: isize,
            y: isize,
            z: isize,
        }
        vec.push((x, y, z));
    }
    let mut ans = 0;
    for bit in 0..8 {
        let mut flg = vec![1, 1, 1];
        for i in 0..3 {
            if bit & (1 << i) > 0 {
                flg[i] = -1;
            }
        }
        let mut vec2 = Vec::new();
        for i in 0..N {
            let x = vec[i].0;
            let y = vec[i].1;
            let z = vec[i].2;
            let s = x * flg[0] + y * flg[1] + z * flg[2];
            vec2.push((s, x, y, z));
        }
        vec2.sort_by(|a, b| b.0.cmp(&a.0));
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;
        for i in 0..M {
            x += vec2[i].1;
            y += vec2[i].2;
            z += vec2[i].3;
        }
        let res = x.abs() + y.abs() + z.abs();
        ans = max(ans, res);
    }
    println!("{}", ans);
}