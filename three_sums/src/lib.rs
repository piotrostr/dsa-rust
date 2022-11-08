use std::collections::{HashMap, HashSet};

/// # Given an integer array nums
/// ## Problem
///
/// Return all the triplets `[nums[i], nums[j], nums[k]]` such that `i != j`,
/// `i != k`, `j != k` and `nums[i] + nums[j] + nums[k] == 0`.
///
/// ## Examples
/// * Input: nums = `[-1,0,1,2,-1,-4]`
/// * Output: `[[-1,-1,2],[-1,0,1]]`
/// * Explanation:
///   * nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0
///   * nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0
///   * nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0
/// The distinct triplets are `[-1,0,1]` and `[-1,-1,2]`.
/// Notice that the order of the output and the order of the triplets does not
/// matter.
///
/// * Input: nums = `[0,1,1]`
/// * Output: []
/// * Explanation: The only possible triplet does not sum up to 0.
///
/// * Input: nums = `[0,0,0]`
/// * Output: `[[0,0,0]]`
/// * Explanation: The only possible triplet sums up to 0.
///
/// ## Constraints
/// * `3 <= nums.length <= 3000`
/// * `-105 <= nums[i] <= 105`
///
/// ## Solution
/// iterate through all of the nums with three pointers would be O(n^3),
/// checking the condition for every possible combination, even repeating the
/// indices:
///
/// for i..nums.len() {
///   for j..nums.len() {
///     for k..nums.len() {
///       ...
///     }
///   }
/// }
///
/// pretty bad
///
/// getting all of the combinations of unique indices and going through those -
/// also bad since constraint is <= 3000, there will be way way way too many
/// indices to find all the possible O(factorial - the combinations that won't
/// occur due to uniqueness)
///
/// The solution for finding sums with 2 index pointers in a sorted array is O(n)
///
/// It could be used with the third index being binary searched on every
/// potential triplet
///
/// get vec
///
/// [-1, 0, 1, 2, -1, -4]
///
/// sort the vec
///
/// [-4, -1, -1, 0, 1, 2]
///
/// create two indices, i and j put them at two ends of array
///
/// -4 + 2 = -2
///
/// search for -2 in the array,
///
/// if doesn't exist continue by sliding both indices
/// if it exists, check if it satisfying the condition
///
/// slide the indices through the whole array, until reaching equality or
/// pointers crossing
///
/// I think that there is no need for a double loop since
/// there is the binary search for the triplet, might be coded later
///
/// Update: having been hinted that this is just `two_sum` with an additional
/// number the problem becomes simple
///
/// Solution would be to simply iterate through the array having fixed on very
/// number and then performing two sums!
///
/// Lets start with implementing the non-optimal algorithm with re-creating a
/// new hash map on every run O(n^2) for both complexity and space
///
/// The twist is that the indices cannot be the same, so got to ensure that too later
pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            map.insert(*num, index as i32);
        }
        let mut res: HashSet<Vec<i32>> = HashSet::new();
        for (i, num) in nums.iter().enumerate() {
            // so that two_sum + num = 0
            // meaning that two_sum = -num
            // the `num` and the two more nums have to be zero
            // meaning, that the target is the opposite of the number
            let target = -num;
            for possible_match in (&two_sum(&map, nums.clone(), target)).iter() {
                if let [j, k] = possible_match[..] {
                    let _j = j as usize;
                    let _k = k as usize;
                    if check_condition(&nums, i, _j, _k) {
                        // push sorted, then hashset will automatically de-dup
                        let mut subres = vec![nums[i], nums[_j], nums[_k]];
                        subres.sort();
                        res.insert(subres);
                    }
                }
            }
        }
        let mut resvec = Vec::from_iter(res.into_iter());
        resvec.sort();
        return resvec;
    }
}

pub fn two_sum(map: &HashMap<i32, i32>, nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    for (index, num) in nums.iter().enumerate() {
        match map.get(&(target - *num)) {
            Some(&complement_index) => {
                res.push(vec![index as i32, complement_index]);
            }
            None => {}
        };
    }

    return res;
}

pub fn check_condition(nums: &Vec<i32>, i: usize, j: usize, k: usize) -> bool {
    if i != j && i != k && j != k && nums[i] + nums[j] + nums[k] == 0 {
        return true;
    }
    return false;
}

pub fn deduplicate<T: std::cmp::Eq + std::hash::Hash>(nums: Vec<T>) -> Vec<T> {
    // convert to hash set
    let unique: HashSet<T> = HashSet::from_iter(nums.into_iter());
    // convert the de-duped back into vector
    return Vec::from_iter(unique);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deduplicate_works() {
        let inp = vec![vec![1, 2, 3], vec![1, 2, 3]];
        let want = vec![vec![1, 2, 3]];
        let got = deduplicate(inp.clone());
        println!("inp {:?} got {:?}", inp, got);
        assert_eq!(got, want);
    }

    #[test]
    fn example_1() {
        let got = Solution::three_sum([-1, 0, 1, 2, -1, -4].to_vec());
        let want = [[-1, -1, 2].to_vec(), [-1, 0, 1].to_vec()].to_vec();
        assert_eq!(got, want);
    }

    #[test]
    fn example_2() {
        let got = Solution::three_sum([0, 1, 1].to_vec());
        let want: Vec<Vec<i32>> = [].to_vec();
        assert_eq!(got, want);
    }

    #[test]
    fn example_3() {
        let got = Solution::three_sum([0, 0, 0].to_vec());
        let want = [[0, 0, 0].to_vec()].to_vec();
        assert_eq!(got, want);
    }

    #[test]
    fn example_4() {
        let inp = [-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4];
        let got = Solution::three_sum(inp.to_vec());
        let want = [
            [-4, 0, 4],
            [-4, 1, 3],
            [-3, -1, 4],
            [-3, 0, 3],
            [-3, 1, 2],
            [-2, -1, 3],
            [-2, 0, 2],
            [-1, -1, 2],
            [-1, 0, 1],
        ]
        .map(|sub| sub.to_vec())
        .to_vec();
        assert_eq!(got, want);
    }
}
