use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};

fn main() {
    input!{
        a: i32
    }

    let ans:i32 = a + a* a + a*a*a;

    println!("{}", ans);
}
