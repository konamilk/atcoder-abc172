use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
use num::integer::sqrt;

fn main() {
    input! {
        n:usize
    }

    let mut yaku_tbl = vec![2;n+1];

    yaku_tbl[0] = 0;
    yaku_tbl[1] = 1;

    for i in 2..=n {
        for j in 2..=n/i{
            yaku_tbl[j * i] += 1;
        }
    }

    let mut ans = 0i64;

    for i in 1..=n{
        ans += (i as i64) * (yaku_tbl[i] as i64)
    }

    println!("{}", ans);
}
