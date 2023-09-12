// https://atcoder.jp/contests/abc134/tasks/abc134_e

use proconio::input;
use proconio::fastout;

const INF: usize = 1<<60;

use std::cmp::Ordering;
fn bisect_left<T: Ord>(vec: &[T], v: &T) -> usize {
    vec.binary_search_by(|x| {
        if x < v {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }).unwrap_or_else(|x| x)
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut LIS = vec![INF; N+1];
    for i in (0..N).rev() {
        let idx = bisect_left(&LIS, &(A[i]+1));
        LIS[idx] = A[i];
    }
    for i in 1..=N {
        if LIS[i] == INF {
            println!("{}", i);
            return;
        }
    }
}