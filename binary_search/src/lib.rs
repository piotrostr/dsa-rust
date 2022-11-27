pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let left = 0;
        let right = nums.len() - 1;
        return Solution::binary_search(nums, left, right, target);
    }

    pub fn binary_search(nums: Vec<i32>, left: usize, right: usize, target: i32) -> i32 {
        if left > right {
            return -1;
        }

        let mid = ((left + right) as f32 / 2.).ceil() as usize;

        if nums[mid] == target {
            return mid as i32;
        }

        // prevent overflow, usize lowest value is 0
        if mid == 0 {
            return -1;
        }

        if nums[mid] > target {
            // take the left side
            return Solution::binary_search(nums, left, mid - 1, target);
        }

        if nums[mid] < target {
            // take the right side
            return Solution::binary_search(nums, mid + 1, right, target);
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        let result = Solution::search(nums, target);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        let result = Solution::search(nums, target);
        assert_eq!(result, -1);
    }

    #[test]
    fn example_3() {
        let nums = vec![2, 5];
        let target = 0;
        let result = Solution::search(nums, target);
        assert_eq!(result, -1);
    }

    #[test]
    fn example_4() {
        let nums = vec![2, 5];
        let target = 5;
        let result = Solution::search(nums, target);
        assert_eq!(result, 1);
    }
}
