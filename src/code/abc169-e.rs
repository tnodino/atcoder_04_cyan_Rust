// https://atcoder.jp/contests/abc169/tasks/abc169_e

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut A = Vec::new();
    let mut B = Vec::new();
    for _ in 0..N {
        input! {
            (a, b): (f64, f64),
        }
        A.push(a);
        B.push(b);
    }
    A.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    B.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    let (L, R) = match N % 2 {
        0 => ((A[N/2-1] + A[N/2]) / 2., (B[N/2-1] + B[N/2]) / 2.),
        1 => (A[N/2], B[N/2]),
        _ => unreachable!(),
    };
    println!("{}", match N % 2 {
        0 => (R - L) / 0.5 + 1.,
        1 => R - L + 1.,
        _ => unreachable!(),
    });
}