// https://atcoder.jp/contests/abc126/tasks/abc126_d

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

const INF: usize = 1<<60;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut G = vec![Vec::new(); N];
    for _ in 0..N-1 {
        input! {
            u: usize,
            v: usize,
            w: usize,
        }
        G[u-1].push((v-1, w));
        G[v-1].push((u-1, w));
    }
    let mut cost = vec![INF; N];
    cost[0] = 0;
    let mut que = VecDeque::new();
    que.push_back(0);
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for (nxt, w) in G[pos].iter() {
            if cost[*nxt] == INF {
                cost[*nxt] = cost[pos] + w;
                que.push_back(*nxt);
            }
        }
    }
    for i in 0..N {
        println!("{}", cost[i] % 2);
    }
}