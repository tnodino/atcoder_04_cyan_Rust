// https://atcoder.jp/contests/abc098/tasks/arc098_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut l = 0;
    let mut xor = 0;
    let mut sum = 0;
    let mut ans = 0;
    for r in 0..N {
        xor ^= A[r];
        sum += A[r];
        while l < r && xor != sum {
            xor ^= A[l];
            sum -= A[l];
            l += 1;
        }
        ans += r - l + 1;
    }
    println!("{}", ans);
}