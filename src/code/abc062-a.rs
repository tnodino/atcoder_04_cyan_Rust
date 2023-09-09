// https://atcoder.jp/contests/abc062/tasks/arc074_a

use proconio::input;
use proconio::fastout;
use std::cmp::{max, min};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    let M = H * W;
    let mut ans = 1<<60;
    for i in 1..H {
        let a = i * W;
        let m = M - a;
        let x = H - i;
        let y = W;
        let b = x / 2 * y;
        let c = m - b;
        let ma = max(a, max(b, c));
        let mi = min(a, min(b, c));
        ans = min(ans, ma - mi);
        let b = y / 2 * x;
        let c = m - b;
        let ma = max(a, max(b, c));
        let mi = min(a, min(b, c));
        ans = min(ans, ma - mi);
    }
    for j in 1..W {
        let a = j * H;
        let m = M - a;
        let x = H;
        let y = W - j;
        let b = x / 2 * y;
        let c = m - b;
        let ma = max(a, max(b, c));
        let mi = min(a, min(b, c));
        ans = min(ans, ma - mi);
        let b = y / 2 * x;
        let c = m - b;
        let ma = max(a, max(b, c));
        let mi = min(a, min(b, c));
        ans = min(ans, ma - mi);
    }
    println!("{}", ans);
}