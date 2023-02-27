pub struct Solution {}

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut num_map = HashMap::new();
        for num in nums.into_iter() {
            if num == 0 {
                continue;
            }
            num_map.entry(num).or_insert(0);
        }
        return num_map.len() as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_operations_test() {
        assert_eq!(Solution::minimum_operations(vec![1, 2, 3, 4, 5]), 5);
    }
}
