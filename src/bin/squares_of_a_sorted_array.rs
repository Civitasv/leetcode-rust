fn main() {}
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut lo: usize = 0;
    let mut hi: usize = nums.len() - 1;
    let mut pos = hi;
    let mut res = vec![0; nums.len()];
    while lo <= hi {
        let a = nums[lo];
        let b = nums[hi];

        if a * a > b * b {
            res[pos] = a * a;
            lo += 1;
        } else {
            res[pos] = b * b;
            if hi == 0 {
                break;
            }
            hi -= 1;
        }
        pos -= 1;
    }

    return res;
}
