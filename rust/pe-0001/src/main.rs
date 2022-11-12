// Project Euler 0001
// Find the sum of all the multiples of 3 or 5 below 1000.

fn main() {
    let mut i: u32 = 0;
    let mut sum: u32 = 0;

    while i < 1000 {
        sum += if i % 3 == 0 || i % 5 == 0 { i } else { 0 };
        i += 1;
    }

    println!("{sum}");
}
