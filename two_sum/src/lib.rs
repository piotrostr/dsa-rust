use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // dump the numbers into a map
        let mut map = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            map.insert(num, index);
        }

        for (index, num) in nums.iter().enumerate() {
            let complement = target - num;
            match map.get(&complement) {
                Some(complement_index) => {
                    if *complement_index != index {
                        return vec![index as i32, *complement_index as i32];
                    }
                }
                None => {}
            }
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
        let got = Solution::two_sum(nums, target);
        let want = [0, 1];
        assert_eq!(got, want);
    }

    #[test]
    fn example_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let got = Solution::two_sum(nums, target);
        let want = [1, 2];
        assert_eq!(got, want);
    }

    #[test]
    fn example_3() {
        let nums = vec![3, 3];
        let target = 6;
        let got = Solution::two_sum(nums, target);
        let want = [0, 1];
        assert_eq!(got, want);
    }
}
