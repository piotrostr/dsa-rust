use std::collections::HashMap;
/// Keep a hash map of the number of occurrences, keep an array of the top
/// occurences at any given time
///
/// On every next `num`, increment occurrences in the map
///
/// once have the occurences find the top `k` occurrences using a binary heap
///
/// automatically, it keeps the largest number in the root
pub struct Solution {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut occurrences = HashMap::<i32, i32>::new();
        for num in nums.clone() {
            occurrences.entry(num).and_modify(|v| *v += 1).or_insert(1);
        }
        // collect into vec of tuples to be sortable
        let mut values = occurrences.into_iter().collect::<Vec<(i32, i32)>>();

        // tuples are (num, occurences): &(i32, i32)
        // sort so that the values are in the order from the most occuring to least occuring
        values.sort_by(|a, b| b.1.cmp(&a.1));

        return values.iter().take(k as usize).map(|x| x.0).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let result = Solution::top_k_frequent(nums, k);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn example_2() {
        let nums = vec![1];
        let k = 1;
        let result = Solution::top_k_frequent(nums, k);
        assert_eq!(result, vec![1]);
    }
}
