#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::cmp::min;
#[fastout]
fn main() {
    input! {
        n: isize,
        a: [isize; n]
    }
    let a_sum = a.iter().sum::<isize>();
    let a_ave = a_sum / n - 1;
    let a_pow = a.iter().map(|x| x * x).sum::<isize>();
    let mut score = std::isize::MAX;
    for i in 0..3 {
        score = min(
            score,
            a_pow - 2 * a_sum * (a_ave + i) + n * (a_ave + i) * (a_ave + i),
        );
    }
    println!("{}", score);
}
