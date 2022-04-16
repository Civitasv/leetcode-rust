use std::collections::HashMap;

fn main() {
    let nums = vec![3, 2, 4];
    let res = two_sum(nums, 6);

    assert_eq!(res, vec![1, 2]);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        if hash_map.contains_key(&(target - *num)) {
            let index2 = hash_map.get(&(target - *num)).unwrap();
            return vec![index as i32, *index2];
        }
        hash_map.insert(*num as i32, index as i32);
    }
    vec![]
}
