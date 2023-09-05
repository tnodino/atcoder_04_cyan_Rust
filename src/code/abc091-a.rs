// https://atcoder.jp/contests/abc091/tasks/arc092_a

use proconio::input;
use proconio::fastout;

const INF: isize = 1<<50;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut red = Vec::new();
    let mut blue = Vec::new();
    for _ in 0..N {
        input! {
            (a, b): (isize, isize),
        }
        red.push((a, b));
    }
    for _ in 0..N {
        input! {
            (c, d): (isize, isize),
        }
        blue.push((c, d));
    }
    blue.sort();
    let mut ans = 0;
    for b in 0..N {
        let mut ma = -1;
        let mut idx = 0;
        for r in 0..N {
            if red[r].0 < blue[b].0 && red[r].1 < blue[b].1 && ma < red[r].1 {
                ma = red[r].1;
                idx = r;
            }
        }
        if ma != -1 {
            ans += 1;
            red[idx] = (INF, INF);
        }
    }
    println!("{}", ans);
}