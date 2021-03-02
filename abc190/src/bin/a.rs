use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      a: usize,
      b: usize,
      c: usize,
    }

    if a + c > b {
        println!("Takahashi")
    } else {
        println!("Aoki")
    }
}
