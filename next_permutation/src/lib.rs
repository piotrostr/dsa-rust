pub struct Solution {}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) -> Vec<i32> {
        return nums.to_vec();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![1, 2, 3];
        let want = vec![1, 3, 2];
        let got = Solution::next_permutation(&mut nums);
        assert_eq!(got, want);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![3, 2, 1];
        let want = vec![1, 2, 3];
        let got = Solution::next_permutation(&mut nums);
        assert_eq!(got, want);
    }

    #[test]
    fn example_3() {
        let mut nums = vec![1, 1, 5];
        let want = vec![1, 5, 1];
        let got = Solution::next_permutation(&mut nums);
        assert_eq!(got, want);
    }
}
