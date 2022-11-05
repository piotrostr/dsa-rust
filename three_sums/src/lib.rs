/// Problem:
/// Given an integer array nums
///
/// return all the triplets [nums[i], nums[j], nums[k]]
///     such that
///     i != j,
///     i != k,
///     j != k
///       and
///     nums[i] + nums[j] + nums[k] == 0.
///
/// Input: nums = [-1,0,1,2,-1,-4]
/// Output: [[-1,-1,2],[-1,0,1]]
/// Explanation:
///     nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
///     nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
///     nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
/// The distinct triplets are [-1,0,1] and [-1,-1,2].
/// Notice that the order of the output and the order of the triplets does not
/// matter.
///
/// Input: nums = [0,1,1]
/// Output: []
/// Explanation: The only possible triplet does not sum up to 0.
///
/// Input: nums = [0,0,0]
/// Output: [[0,0,0]]
/// Explanation: The only possible triplet sums up to 0.
///
/// Constraints:
///     3 <= nums.length <= 3000
///     -105 <= nums[i] <= 105
pub struct Solution {}

impl Solution {
    pub fn three_sums(nums: Vec<i32>) -> Vec<Vec<i32>> {
        return Vec::from(Vec::new());
    }

    pub fn condition(nums: Vec<i32>, i: usize, j: usize, k: usize) -> bool {
        if i != j && i != k && j != k && nums[i] + nums[j] + nums[k] == 0 {
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let got = Solution::three_sums([-1, 0, 1, 2, -1, -4].to_vec());
        let want = [[-1, -1, 2].to_vec(), [-1, 0, 1].to_vec()].to_vec();
        assert_eq!(got.clone().sort(), want.clone().sort());
    }

    #[test]
    fn example_2() {
        let got = Solution::three_sums([0, 1, 1].to_vec());
        let want: Vec<Vec<i32>> = [].to_vec();
        assert_eq!(got.clone().sort(), want.clone().sort());
    }

    #[test]
    fn example_3() {
        let got = Solution::three_sums([0, 0, 0].to_vec());
        let want = [[0, 0, 0].to_vec()].to_vec();
        assert_eq!(got.clone().sort(), want.clone().sort());
    }
}
