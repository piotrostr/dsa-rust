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
/// lets try also a bigger array, just two or three permutations
///
/// Example for array `[5, 3, 5, 1, 1, 5, 9, 1, 5]`
///
/// `[5, 3, 5, 1, 1, 5, 1, 9, 5]`
///
/// always start with the right end, as this is the smallest influence
/// 200 > 120 > 111
///
/// start from the right end
/// if array[-2] < array[-1]:
///     swap the array[-1], array[-2]
/// while greater:
///     continue to find smaller; if smaller swap the array[-1]
/// if no smaller, pop the array[-1] and insert at the beginning of array
///
pub struct Solution {}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) -> Vec<i32> {
        let last_num = nums[nums.len() - 1].clone();
        let mut nums = VecDeque::from(nums.clone());

        // start with the second last index, go until `index = 0`
        // the range is exclusive, thus `nums.len() + 1`
        for i in 2..(nums.len() + 1) {
            let index = nums.len() - i;
            if nums[index] < last_num {
                let num_at_index = nums[index];
                nums[index] = last_num;
                let length_of_nums = nums.len();
                nums[length_of_nums - 1] = num_at_index;
                return Vec::from(nums);
            }
        }
        nums.pop_back();
        nums.push_front(last_num);
        return Vec::from(nums);
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
        assert_eq!(got, want, "next_permutation({:?})", nums);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![3, 2, 1];
        let want = vec![1, 2, 3];
        let got = Solution::next_permutation(&mut nums);
        assert_eq!(got, want, "next_permutation({:?})", nums);
    }

    #[test]
    fn example_3() {
        let mut nums = vec![1, 1, 5];
        let want = vec![1, 5, 1];
        let got = Solution::next_permutation(&mut nums);
        assert_eq!(got, want, "next_permutation({:?})", nums);
    }

    #[test]
    fn example_4() {
        let mut nums = vec![1, 3, 2];
        let want = vec![2, 1, 3];
        let got = Solution::next_permutation(&mut nums);
        assert_eq!(got, want, "next_permutation({:?})", nums);
    }

    #[test]
    fn example_5() {
        let mut nums = vec![3, 1, 2];
        let want = vec![3, 2, 1];
        let got = Solution::next_permutation(&mut nums);
        assert_eq!(got, want);
    }

    #[test]
    fn example_6() {
        let mut nums = vec![2, 1, 3];
        let want = vec![2, 3, 1];
        let got = Solution::next_permutation(&mut nums);
        assert_eq!(got, want);
    }

    // #[test]
    // skip for now
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
