// https://atcoder.jp/contests/abc123/tasks/abc123_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        Y: usize,
        Z: usize,
        K: usize,
        A: [usize; X],
        B: [usize; Y],
        C: [usize; Z],
    }
    let mut vec1 = Vec::new();
    for i in 0..X {
        for j in 0..Y {
            vec1.push(A[i] + B[j]);
        }
    }
    vec1.sort_by(|a, b| b.cmp(&a));
    if vec1.len() > K {
        vec1 = vec1[..K].to_vec();
    }
    let mut vec2 = Vec::new();
    for i in 0..vec1.len() {
        for j in 0..Z {
            vec2.push(vec1[i] + C[j]);
        }
    }
    vec2.sort_by(|a, b| b.cmp(&a));
    for i in 0..K {
        println!("{}", vec2[i]);
    }
}