pub struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        vec![]
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
