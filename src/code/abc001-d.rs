// https://atcoder.jp/contests/abc001/tasks/abc001_4

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = 90_000;
    let mut time = vec![false; M];
    for _ in 0..N {
        input! {
            s: String,
        }
        let mut sp = s.split("-");
        let S = sp.next().unwrap();
        let E = sp.next().unwrap();
        let mut S = S.parse::<usize>().unwrap();
        let mut E = E.parse::<usize>().unwrap();
        S = (S / 100 * 60 + S % 100) / 5 * 5;
        E = (E / 100 * 60 + E % 100 + 4) / 5 * 5;
        for i in S..=E {
            time[i] = true;
        }
    }
    let mut flg = false;
    for i in 0..M {
        if time[i] {
            if !flg {
                print!("{:04}-", i / 60 * 100 + i % 60);
            }
        }
        else {
            if flg {
                println!("{:04}", (i - 1) / 60 * 100 + (i - 1) % 60);
            }
        }
        flg = time[i];
    }
}