fn main() {}
struct Solution;
impl Solution {
    pub fn is_bad_version(&self, version: i32) -> bool {
        return true;
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut lo = 1;
        let mut hi = n;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let is_bad = self.is_bad_version(mid);
            if is_bad {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }

        return lo;
    }
}
