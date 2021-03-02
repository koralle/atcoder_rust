use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: usize,
      m: usize,
      ab: [(usize, usize); m],
      k: usize,
      cd: [(usize, usize); k],
    }

    let mut ans = 0;
    for bit in 0..(1 << k) {
        let mut balls = vec![false; n + 1];
        for i in 0..k {
            let (c, d) = cd[i];
            if bit & (1 << i) > 0 {
                balls[c] = true
            } else {
                balls[d] = true
            }
        }

        let mut cnt = 0;
        for j in 0..m {
            let (a, b): (usize, usize) = ab[j];
            if balls[a] && balls[b] {
                cnt += 1
            }
        }

        ans = ans.max(cnt);
    }
    println!("{}", ans)
}
