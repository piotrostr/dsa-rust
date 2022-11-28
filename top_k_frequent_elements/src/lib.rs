impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let result = Solution::top_k_frequent(nums, k);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn example_2() {
        let nums = vec![1];
        let k = 1;
        let result = Solution::top_k_frequent(nums, k);
        assert_eq!(result, vec![1]);
    }
}
