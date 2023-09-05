// https://atcoder.jp/contests/abc067/tasks/arc078_b

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

const INF: usize = 1<<60;

#[allow(non_snake_case)]
fn bfs(s: usize, N: usize, G: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut dist = vec![INF; N];
    dist[s] = 0;
    let mut que = VecDeque::from([s]);
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for nxt in G[pos].iter() {
            if dist[pos] + 1 < dist[*nxt] {
                dist[*nxt] = dist[pos] + 1;
                que.push_back(*nxt);
            }
        }
    }
    return dist;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut G = vec![Vec::new(); N];
    for _ in 0..N-1 {
        input! {
            (a, b): (usize, usize),
        }
        G[a-1].push(b-1);
        G[b-1].push(a-1);
    }
    let black = bfs(0, N, &G);
    let white = bfs(N-1, N, &G);
    let mut cnt = 0;
    for i in 0..N {
        if black[i] <= white[i] {
            cnt += 1;
        }
    }
    println!("{}", match cnt > N / 2 {
        true => "Fennec",
        false => "Snuke",
    });
}