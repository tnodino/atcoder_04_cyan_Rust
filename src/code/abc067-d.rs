// https://atcoder.jp/contests/abc067/tasks/arc078_b

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
            a: usize,
            b: usize,
        }
        G[a-1].push(b-1);
        G[b-1].push(a-1);
    }
    let mut b = vec![INF; N];
    b[0] = 0;
    let mut que = VecDeque::from([0]);
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for nxt in G[pos].iter() {
            if b[pos] + 1 < b[*nxt] {
                b[*nxt] = b[pos] + 1;
                que.push_back(*nxt);
            }
        }
    }
    let mut w = vec![INF; N];
    w[N-1] = 0;
    let mut que = VecDeque::from([N-1]);
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for nxt in G[pos].iter() {
            if w[pos] + 1 < w[*nxt] {
                w[*nxt] = w[pos] + 1;
                que.push_back(*nxt);
            }
        }
    }
    let mut cnt = 0;
    for i in 0..N {
        if b[i] <= w[i] {
            cnt += 1;
        }
    }
    println!("{}", match cnt > N / 2 {
        true => "Fennec",
        false => "Snuke",
    });
}