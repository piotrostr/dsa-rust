use std::collections::VecDeque;

/// The solution requires O(1) memory usage, the replacement has to be in-place
/// Example of all of the permutationn for array `[1,2,3]`
///
/// `[1, 2, 3]`
/// `[1, 3, 2]`
/// `[2, 1, 3]`
/// `[2, 3, 1]`
/// `[3, 1, 2]`
/// `[3, 2, 1]`
///
/// the array is sorted, indices go
///
/// `[0, 1, 2]`
/// `[0, 2, 1]`
/// `[1, 0, 2]`
/// `[1, 2, 0]`
/// `[2, 0, 1]`
/// `[2, 1, 0]`
///
/// essentially you like *pull to the left*
///
/// meaning, if sorted, pop() push()
pub struct Solution {}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) -> Vec<i32> {
        if is_sorted(nums.to_vec()) {
            let mut nums = VecDeque::from(nums.to_vec());
            let first_num = nums.pop_front().unwrap();
            nums.push_back(first_num);
            return Vec::from(nums);
        }

        return nums.to_vec();
    }
}

pub fn is_sorted(nums: Vec<i32>) -> bool {
    return is_sorted_asc(nums.clone()) || is_sorted_desc(nums.clone());
}

/// algorithmic complexity in worst case scenario O(n)
pub fn is_sorted_asc(nums: Vec<i32>) -> bool {
    // incrementing case
    let mut num_before = nums[0];
    for &num in nums[1..].iter() {
        println!("{}", num);
        if num < num_before {
            return false;
        }
        num_before = num;
    }

    return true;
}

/// algorithmic complexity in worst case scenario O(n)
pub fn is_sorted_desc(nums: Vec<i32>) -> bool {
    // decrementing case
    let mut num_before = nums[0];
    for &num in nums[1..].iter() {
        println!("{}", num);
        if num > num_before {
            return false;
        }
        num_before = num;
    }

    return true;
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

    #[test]
    fn is_sorted_works() {
        let cases = &vec![vec![1, 1, 5], vec![3, 2, 1], vec![1, 2, 3]];
        let wants = &vec![true, true, true];

        for (case, want) in cases.iter().zip(wants.iter()) {
            let case = case.clone();
            let want = want.clone();
            let got = is_sorted(case.clone());
            assert_eq!(want, got, "is_sorted({:?}) -> {} != {}", case, got, want);
        }
    }
}
