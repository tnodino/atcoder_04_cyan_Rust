// https://atcoder.jp/contests/abc104/tasks/abc104_c

use proconio::input;
use std::cmp::min;

#[allow(non_snake_case)]
fn main() {
    input! {
        D: usize,
        G: usize,
    }
    let mut p = Vec::new();
    let mut c = Vec::new();
    for _ in 0..D {
        input! {
            a: usize,
            b: usize,
        }
        p.push(a);
        c.push(b);
    }
    let mut ans = 1<<60;
    'outer: for bit in 0..1<<D {
        let mut res = 0;
        let mut s = 0;
        for i in 0..D {
            if bit & (1 << i) > 0 {
                res += p[i];
                s += (i + 1) * 100 * p[i] + c[i];
            }
        }
        if s >= G {
            ans = min(ans, res);
            continue 'outer;
        }
        for i in (0..D).rev() {
            if bit & (1 << i) > 0 {
                continue;
            }
            for _ in 0..p[i] {
                res += 1;
                s += (i + 1) * 100;
                if s >= G {
                    ans = min(ans, res);
                    continue 'outer;
                }
            }
        }
    }
    println!("{}", ans);
}