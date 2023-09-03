// https://atcoder.jp/contests/abc176/tasks/abc176_e

use proconio::input;
use proconio::fastout;
use std::collections::{HashMap, HashSet};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _H: usize,
        _W: usize,
        M: usize,
    }
    let mut set = HashSet::new();
    let mut cnth = HashMap::new();
    let mut cntw = HashMap::new();
    for _ in 0..M {
        input! {
            h: usize,
            w: usize,
        }
        set.insert((h, w));
        *cnth.entry(h).or_insert(0) += 1;
        *cntw.entry(w).or_insert(0) += 1;
    }
    let maxh = *cnth.values().max().unwrap();
    let maxw = *cntw.values().max().unwrap();
    let mut canh = Vec::new();
    let mut canw = Vec::new();
    for (k, v) in cnth.iter() {
        if maxh == *v {
            canh.push(*k);
        }
    }
    for (k, v) in cntw.iter() {
        if maxw == *v {
            canw.push(*k);
        }
    }
    for h in canh.iter() {
        for w in canw.iter() {
            if !set.contains(&(*h, *w)) {
                println!("{}", maxh + maxw);
                return;
            }
        }
    }
    println!("{}", maxh + maxw - 1);
}