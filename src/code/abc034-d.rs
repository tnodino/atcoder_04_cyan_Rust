// https://atcoder.jp/contests/abc034/tasks/abc034_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut w = Vec::new();
    let mut p = Vec::new();
    for _ in 0..N {
        input! {
            a: f64,
            b: f64,
        }
        w.push(a);
        p.push(b / 100.);
    }
    let mut ok = 0.;
    let mut ng = 1.;
    for _ in 0..100 {
        let x = (ok + ng) / 2.;
        let mut v = Vec::new();
        for i in 0..N {
            v.push(w[i] * (p[i] - x));
        }
        v.sort_by(|a, b| b.partial_cmp(a).unwrap());
        if 0. <= v[..K].iter().sum::<f64>() {
            ok = x;
        }
        else {
            ng = x;
        }
    }
    println!("{}", ok * 100.);
}