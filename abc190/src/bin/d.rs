use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }

    let mut index: u64 = 1;

    let mut ans: u64 = 0;
    let mut y1 = Vec::<u64>::new();
    let mut y2 = Vec::<u64>::new();
    while index * index <= 2 * n {

        if (2 * n) % index == 0 {
            if index != ((2 * n) / index) {
                y1.push(index);
                y2.push((2 * n) / index);
            }
        }

        index += 1
    }

    for idx in 0..y1.len() {
        if ((y1[idx] % 2) == 0) ^ ((y2[idx] % 2) == 0) {
            ans += 2
        }
    }

    println!("{}", ans)
}
