// https://atcoder.jp/contests/abc087/tasks/arc090_b

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut G = vec![Vec::new(); N];
    let mut vec = Vec::new();
    for _ in 0..M {
        input! {
            L: usize,
            R: usize,
            D: isize,
        }
        G[L-1].push((R-1, D));
        G[R-1].push((L-1, -D));
        vec.push((L-1, R-1, D));
    }
    let mut cost = vec![0; N];
    let mut flg = vec![false; N];
    for i in 0..N {
        if flg[i] {
            continue;
        }
        let mut que = VecDeque::new();
        que.push_back(i);
        flg[i] = false;
        while !que.is_empty() {
            let pos = que.pop_front().unwrap();
            for (nxt, d) in G[pos].iter() {
                if !flg[*nxt] {
                    cost[*nxt] = cost[pos] + d;
                    flg[*nxt] = true;
                    que.push_back(*nxt);
                }
            }
        }
    }
    for i in 0..M {
        if cost[vec[i].0] + vec[i].2 != cost[vec[i].1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}