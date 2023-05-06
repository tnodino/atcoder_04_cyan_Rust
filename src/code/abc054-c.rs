// https://atcoder.jp/contests/abc054/tasks/abc054_c

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn dfs(pos: usize, G: &Vec<Vec<usize>>, flg: &mut Vec<bool>, ans: &mut usize) {
    if flg.iter().all(|&x| x) {
        *ans += 1;
        return;
    }
    for nxt in G[pos].iter() {
        if flg[*nxt] {
            continue;
        }
        flg[*nxt] = true;
        dfs(*nxt, &G, flg, ans);
        flg[*nxt] = false;
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut G = vec![Vec::new(); N];
    for _ in 0..M {
        input! {
            a: usize,
            b: usize,
        }
        G[a-1].push(b-1);
        G[b-1].push(a-1);
    }
    let mut flg = vec![false; N];
    flg[0] = true;
    let mut ans = 0;
    dfs(0, &G, &mut flg, &mut ans);
    println!("{}", ans);
}