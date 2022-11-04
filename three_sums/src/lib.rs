pub struct Solution {}

impl Solution {
    pub fn three_sums(nums: Vec<i32>) -> Vec<Vec<i32>> {
        return [[1, -2, 3].to_vec(), [4, 5, 6].to_vec()].to_vec();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let got = Solution::three_sums([-1, 0, 1, 2, -1, -4].to_vec());
        let want = [[-1, -1, 2].to_vec(), [-1, 0, 1].to_vec()].to_vec();
        assert_eq!(got, want);
    }

    #[test]
    fn example_2() {
        let got = Solution::three_sums([0, 1, 1].to_vec());
        let want: Vec<Vec<i32>> = [].to_vec();
        assert_eq!(got, want);
    }

    #[test]
    fn example_3() {
        let got = Solution::three_sums([0, 0, 0].to_vec());
        let want = [[0, 0, 0].to_vec()].to_vec();
        assert_eq!(got, want);
    }
}
