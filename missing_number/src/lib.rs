pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums.clone();

        cyclic_sort(&mut nums);

        for (index, num) in nums.iter().enumerate() {
            if index as i32 != *num {
                return index as i32;
            }
        }
        0
    }
}

pub fn cyclic_sort(nums: &mut Vec<i32>) {
    let mut i: usize = 0;
    while i < nums.len() {
        let j = nums[i] as usize;
        if nums[i] < nums.len() as i32 && nums[i] != nums[j] {
            let temp = nums[j];
            nums[j] = nums[i];
            nums[i] = temp;
        } else {
            i += 1;
        }
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
