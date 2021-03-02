use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      s: u64,
      d: u64,
      xy: [(u64, u64); n],
    }

    for i in 0..xy.len() {
        let (x, y) = xy[i];

        if s > x && y > d {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
