fn main() {}

fn move_zeroes(nums: &mut Vec<i32>) {
    let mut lo = 0;
    while lo < nums.len() && nums[lo] != 0 {
        lo += 1;
    }
    let mut hi = lo;
    while hi < nums.len() && nums[hi] == 0 {
        hi += 1;
    }

    while lo < nums.len() && hi < nums.len() {
        if nums[lo] == 0 {
            let temp = nums[lo];
            nums[lo] = nums[hi];
            nums[hi] = temp;
            while hi < nums.len() && nums[hi] == 0 {
                hi += 1;
            }
        }
        lo += 1;
    }
}
