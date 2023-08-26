// https://atcoder.jp/contests/abc176/tasks/abc176_d

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

const DX1: &[usize] = &[!0, 1, 0, 0];
const DY1: &[usize] = &[ 0, 0,!0, 1];
const DX2: &[usize] = &[!1,!0, 0, 1, 2,!1,!0, 1, 2,!1, 2,!1,!0, 1, 2,!1,!0, 0, 1, 2];
const DY2: &[usize] = &[!1,!1,!1,!1,!1,!0,!0,!0,!0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 2];
const INF: usize = 1<<60;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        Ch: usize,
        Cw: usize,
        Dh: usize,
        Dw: usize,
    }
    let mut S = Vec::new();
    for _ in 0..H {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        S.push(s);
    }
    let mut cost = vec![vec![INF; W]; H];
    cost[Ch-1][Cw-1] = 0;
    let mut que = VecDeque::new();
    que.push_back((Ch-1, Cw-1));
    while !que.is_empty() {
        let (x, y) = que.pop_front().unwrap();
        for d in 0..4 {
            let nx = x.wrapping_add(DX1[d]);
            let ny = y.wrapping_add(DY1[d]);
            if H <= nx || W <= ny {
                continue;
            }
            if S[nx][ny] == '#' {
                continue;
            }
            if cost[x][y] < cost[nx][ny] {
                cost[nx][ny] = cost[x][y];
                que.push_front((nx, ny));
            }
        }
        for d in 0..20 {
            let nx = x.wrapping_add(DX2[d]);
            let ny = y.wrapping_add(DY2[d]);
            if H <= nx || W <= ny {
                continue;
            }
            if S[nx][ny] == '#' {
                continue;
            }
            if cost[x][y] + 1 < cost[nx][ny] {
                cost[nx][ny] = cost[x][y] + 1;
                que.push_back((nx, ny));
            }
        }
    }
    if cost[Dh-1][Dw-1] == INF {
        println!("-1");
    }
    else {
        println!("{}", cost[Dh-1][Dw-1]);
    }
}