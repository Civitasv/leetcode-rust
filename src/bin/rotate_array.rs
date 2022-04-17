fn main() {}

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    fn reverse(nums: &mut Vec<i32>, start: usize, end: usize) {
        let mut start = start;
        let mut end = end;
        while start < end {
            let temp = nums[start];
            nums[start] = nums[end];
            nums[end] = temp;
            start += 1;
            end -= 1;
        }
    }
    let len = nums.len();
    let k = k as usize % len;
    if len < 2 || k == 0 {
        return;
    }
    reverse(nums, 0, len - 1);
    reverse(nums, 0, k - 1);
    reverse(nums, k, len - 1);
}
