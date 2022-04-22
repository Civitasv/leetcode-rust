fn main() {
    println!("{:?}", two_sum(vec![3, 24, 50, 79, 88, 150, 345], 200));
}
fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut lo = 0;
    let mut hi = numbers.len() - 1;
    loop {
        if numbers[lo] + numbers[hi] == target {
            return vec![(lo + 1) as i32, (hi + 1) as i32];
        }
        if numbers[lo] + numbers[hi] < target {
            lo += 1;
        } else {
            hi -= 1;
        }
    }
}
