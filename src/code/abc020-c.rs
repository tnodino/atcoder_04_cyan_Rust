// https://atcoder.jp/contests/abc020/tasks/abc020_c

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

const DX: &[usize] = &[!0, 1, 0, 0];
const DY: &[usize] = &[ 0, 0,!0, 1];
const INF: usize = 1<<60;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        T: usize,
    }
    let mut s = Vec::new();
    for _ in 0..H {
        input! {
            x: String,
        }
        let x = x.chars().collect::<Vec<char>>();
        s.push(x);
    }
    let mut sx = 0;
    let mut sy = 0;
    let mut gx = 0;
    let mut gy = 0;
    for i in 0..H {
        for j in 0..W {
            if s[i][j] == 'S' {
                sx = i;
                sy = j;
                s[i][j] = '.';
            }
            if s[i][j] == 'G' {
                gx = i;
                gy = j;
                s[i][j] = '.';
            }
        }
    }
    let mut ok = 1;
    let mut ng = T;
    for _ in 0..100 {
        let p = (ok + ng) / 2;
        let mut cost = vec![vec![INF; W]; H];
        cost[sx][sy] = 0;
        let mut que = VecDeque::new();
        que.push_back((sx, sy));
        while !que.is_empty() {
            let (x, y) = que.pop_front().unwrap();
            for d in 0..4 {
                let nx = x.wrapping_add(DX[d]);
                let ny = y.wrapping_add(DY[d]);
                if H <= nx || W <= ny {
                    continue;
                }
                if s[nx][ny] == '.' {
                    if cost[x][y] + 1 < cost[nx][ny] {
                        cost[nx][ny] = cost[x][y] + 1;
                        que.push_back((nx, ny));
                    }
                }
                else {
                    if cost[x][y] + p < cost[nx][ny] {
                        cost[nx][ny] = cost[x][y] + p;
                        que.push_back((nx, ny));
                    }
                }
            }
        }
        if cost[gx][gy] <= T {
            ok = p;
        }
        else {
            ng = p;
        }
    }
    println!("{}", ok);
}