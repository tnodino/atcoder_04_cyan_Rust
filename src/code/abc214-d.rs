// https://atcoder.jp/contests/abc214/tasks/abc214_d

use proconio::input;
use proconio::fastout;
use ac_library::Dsu;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut edge = Vec::new();
    for _ in 0..N-1 {
        input! {
            (u, v, w): (usize, usize, usize),
        }
        edge.push((w, u-1, v-1));
    }
    edge.sort();
    let mut UF = Dsu::new(N);
    let mut ans = 0;
    for i in 0..N-1 {
        if !UF.same(edge[i].1, edge[i].2) {
            ans += UF.size(edge[i].1) * UF.size(edge[i].2) * edge[i].0;
        }
        UF.merge(edge[i].1, edge[i].2);
    }
    println!("{}", ans);
}