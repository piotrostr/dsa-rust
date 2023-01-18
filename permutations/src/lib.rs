use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut permutations: HashSet<Vec<i32>> = HashSet::new();
        let target_length = nums.len();
        let mut nums = nums;
        let permutation = vec![];
        for i in 0..nums.len() {
            let mut permutation = permutation.clone();
            // not sure how to go about below in good complexity
            let x = nums.remove(i);
            permutation.push(x);
            bt(&mut permutations, nums.clone(), permutation, &target_length);
            // and here; suboptimal
            nums.insert(i, x);
        }
        let mut res: Vec<Vec<i32>> = permutations.iter().map(|item| item.to_owned()).collect();
        res.sort();
        res
    }
}

pub fn bt(
    permutations: &mut HashSet<Vec<i32>>,
    nums: Vec<i32>,
    permutation: Vec<i32>,
    target_length: &usize,
) {
    if permutation.len() == *target_length {
        permutations.insert(permutation);
        return;
    }
    let mut nums = nums;
    for i in 0..nums.len() {
        // not sure how to go about below in good complexity
        let x = nums.remove(i);
        let mut permutation = permutation.clone().to_owned();
        permutation.push(x);
        bt(
            permutations,
            nums.clone(),
            permutation.clone(),
            target_length,
        );
        // and here; suboptimal
        nums.insert(i, x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            [
                [1, 2, 3],
                [1, 3, 2],
                [2, 1, 3],
                [2, 3, 1],
                [3, 1, 2],
                [3, 2, 1]
            ]
            .map(|arr| arr.to_vec())
            .to_vec()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::permute(vec![0, 1]),
            [[0, 1], [1, 0]].map(|arr| arr.to_vec()).to_vec()
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::permute(vec![1]),
            [[1]].map(|arr| arr.to_vec()).to_vec()
        )
    }
}
