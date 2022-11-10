pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let got = Solution::missing_number(vec![3, 0, 1]);
        let want = 2;
        assert_eq!(got, want);
    }

    #[test]
    fn example_2() {
        let got = Solution::missing_number(vec![3, 0, 1]);
        let want = 2;
        assert_eq!(got, want);
    }

    #[test]
    fn example_3() {
        let got = Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]);
        let want = 8;
        assert_eq!(got, want);
    }
}
