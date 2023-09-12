// https://atcoder.jp/contests/abc074/tasks/arc083_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (A, B, C, D, E, F): (f64, f64, f64, f64, f64, f64),
    }
    let N = 30;
    let M = 3000;
    let mut vec = Vec::new();
    for i in 0..=N {
        for j in 0..=N {
            if i == 0 && j == 0 {
                continue;
            }
            for k in 0..=M {
                for l in 0..=M {
                    let a = i as f64;
                    let b = j as f64;
                    let c = k as f64;
                    let d = l as f64;
                    let water = (A * a + B * b) * 100.;
                    let sugar = C * c + D * d;
                    if water + sugar > F {
                        break;
                    }
                    if (A * a + B * b) * E < sugar {
                        break;
                    }
                    let per = sugar * 100. / (water + sugar);
                    vec.push((per, water + sugar, sugar));
                }
            }
        }
    }
    vec.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    println!("{} {}", vec[0].1, vec[0].2);
}