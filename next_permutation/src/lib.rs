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
/// In order to truly see the motivation behind the algorithm I believe it is
/// important to understand the permutations are like a 'sorted' way of all of
/// outlining all of the permutations between array being sorted ascending and
/// descending
pub struct Solution {}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        // iterating through the left is most intuitive with a loop and
        // substracting index
        let n = nums.len();
        let mut i = n - 2;

        // going to the first decreasing number
        while i > 0 && nums[i] >= nums[i + 1] {
            i -= 1;
        }

        // at this point it is known that the array is sorted, all of the other
        // numbers being greater than the nums[i]
        // to find the solution the cut-off point has to be found - what is the
        // first number from the end greater than four

        // look for the first number above the
        let mut j = n - 1;
        while j > i && nums[j] <= nums[i] {
            j -= 1;
        }

        nums.swap(i, j);
        // since it is known that from index `i`, the values are sorted after
        // the swap, reversing the array will finish the *next permutation*

        // reverse

        // start with the index next to `i`, but only if, then reverse the whole
        // array the inequality check is to check if a `j` index has been found
        if i != j {
            nums[i + 1..n].reverse();
        } else {
            nums[i..n].reverse();
        }
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
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, want);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![3, 2, 1];
        let want = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, want);
    }

    #[test]
    fn example_3() {
        let mut nums = vec![1, 1, 5];
        let want = vec![1, 5, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, want);
    }

    #[test]
    fn example_4() {
        let mut nums = vec![1, 3, 2];
        let want = vec![2, 1, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, want);
    }

    #[test]
    fn example_5() {
        let mut nums = vec![3, 1, 2];
        let want = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, want);
    }

    #[test]
    fn example_6() {
        let mut nums = vec![2, 1, 3];
        let want = vec![2, 3, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, want);
    }

    #[test]
    fn example_7() {
        let mut nums = vec![2, 3, 1];
        let want = vec![3, 1, 2];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, want);
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
