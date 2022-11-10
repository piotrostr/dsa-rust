pub struct Solution;

impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_valid() {
        let first_list = [[0, 2], [5, 10], [13, 23], [24, 25]]
            .map(|sublist| sublist.to_vec())
            .to_vec();
        let second_list = [[1, 5], [8, 12], [15, 24], [25, 26]]
            .map(|sublist| sublist.to_vec())
            .to_vec();
        let result = Solution::interval_intersection(first_list, second_list);
        let want = [[1, 2], [5, 5], [8, 10], [15, 23], [24, 24], [25, 25]]
            .map(|sublist| sublist.to_vec())
            .to_vec();
        assert_eq!(result, want);
    }

    #[test]
    fn it_returns_empty_for_invalid() {
        let first_list = vec![vec![1, 3], vec![5, 9]];
        let second_list = vec![];
        let result = Solution::interval_intersection(first_list, second_list);
        let want: Vec<Vec<i32>> = Vec::new();
        assert_eq!(result, want);
    }
}
