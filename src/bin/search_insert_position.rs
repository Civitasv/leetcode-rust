fn main() {}
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut lo: i32 = 0;
    let mut hi: i32 = (nums.len() - 1) as i32;

    while lo <= hi {
        let mid = (lo + (hi - lo) / 2) as usize;

        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] > target {
            hi = (mid - 1) as i32;
        } else {
            lo = (mid + 1) as i32;
        }
    }

    return lo;
}
