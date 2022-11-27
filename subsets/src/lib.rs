pub struct Solution {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![vec![]];
        for num in nums {
            for i in 1..res.len() {
                let mut subres = res[i].clone();
                subres.push(num);
                res.push(subres);
            }
            res.push(vec![num]);
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![1, 2, 3];
        let mut result = Solution::subsets(nums);
        let mut want = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        want.sort();
        result.sort();
        assert_eq!(result, want);
    }

    #[test]
    fn example_2() {
        let nums = vec![0];
        let result = Solution::subsets(nums);
        let want = vec![vec![], vec![0]];
        assert_eq!(result, want);
    }
}
