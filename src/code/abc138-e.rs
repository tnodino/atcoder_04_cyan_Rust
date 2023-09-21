// https://atcoder.jp/contests/abc138/tasks/abc138_e

use proconio::input;
use proconio::fastout;

use std::cmp::Ordering;
fn bisect_left<T: Ord>(vec: &[T], v: &T) -> usize {
    vec.binary_search_by(|x| {
        if x < v {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }).unwrap_or_else(|x| x)
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String,
        t: String,
    }
    let N = s.len();
    let s = s.chars().collect::<Vec<char>>();
    let mut alp = vec![Vec::new(); 26];
    for i in 0..N {
        let idx = (s[i] as usize) - ('a' as usize);
        alp[idx].push(i);
        alp[idx].push(i + N);
    }
    for i in 0..26 {
        alp[i].sort();
    }
    let mut ans = 0;
    let mut pos = 0;
    for c in t.chars() {
        let idx = (c as usize) - ('a' as usize);
        if alp[idx].is_empty() {
            println!("-1");
            return;
        }
        let jdx = bisect_left(&alp[idx], &pos);
        ans += alp[idx][jdx] - pos + 1;
        pos = (alp[idx][jdx] + 1) % N;
    }
    println!("{}", ans);
}