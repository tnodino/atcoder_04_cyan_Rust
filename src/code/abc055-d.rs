// https://atcoder.jp/contests/abc055/tasks/arc069_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        s: String,
    }
    let mut t = Vec::new();
    for c in s.chars() {
        t.push(match c {
            'o' => 0,
            _ => 1,
        });
    }
    for bit in 0..4 {
        let mut vec = vec![0; N];
        if bit & 1 > 0 {
            vec[0] = 1;
        }
        if bit & 2 > 0 {
            vec[1] = 1;
        }
        for i in 1..N-1 {
            vec[i+1] = vec[i-1] ^ vec[i] ^ t[i];
        }
        if vec[N-2] ^ vec[N-1] ^ vec[0] == t[N-1] && vec[N-1] ^ vec[0] ^ vec[1] == t[0] {
            for i in 0..N {
                print!("{}", match vec[i] {
                    0 => 'S',
                    _ => 'W',
                });
            }
            println!();
            return;
        }
    }
    println!("-1");
}