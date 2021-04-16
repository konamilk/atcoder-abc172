use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};

fn main() {
    input! {
        n:usize,
        m:usize,
        k:i64,
        a:[i64;n],
        b:[i64;m]
    }

    let mut sa = vec![0i64;n+1];
    let mut sb = vec![0i64;m+1];


    let mut first_i = 0usize;

    for i in 0..n{
        sa[i+1] = sa[i] + a[i];
        if sa[i+1] <= k{
            first_i = i+1
        }
    }

    for j in 0..m{
        sb[j+1] = sb[j] + b[j];
    }

    let mut i = first_i;
    let mut j = 0usize;

    let mut ans = first_i;

    // println!("{:?}", sa);
    // println!("{:?}", sb);
    //
    // println!("{} {}", i, j);

    while j < m {
        // println!("{} {}", i, j);
        if sa[i] + sb[j + 1] <= k {
            j = j+1;
            ans = std::cmp::max(ans, i+j);
        }
        else {
            if i == 0{
                break;
            }
            i = i - 1;
        }
    }

    println!("{}", ans);
}
