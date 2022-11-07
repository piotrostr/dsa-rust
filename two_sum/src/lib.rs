use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            match map.get(&(target - *num)) {
                Some(&complement_index) => {
                    return vec![index as i32, complement_index];
                }
                None => map.insert(*num, index as i32),
            };
        }
        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let mut got = Solution::two_sum(nums, target);
        let mut want = [0, 1];
        got.sort();
        want.sort();
        assert_eq!(got, want);
    }

    #[test]
    fn example_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let mut got = Solution::two_sum(nums, target);
        let mut want = [1, 2];
        got.sort();
        want.sort();
        assert_eq!(got, want);
    }

    #[test]
    fn example_3() {
        let nums = vec![3, 3];
        let target = 6;
        let mut got = Solution::two_sum(nums, target);
        let mut want = [0, 1];
        got.sort();
        want.sort();
        assert_eq!(got, want);
    }
}
