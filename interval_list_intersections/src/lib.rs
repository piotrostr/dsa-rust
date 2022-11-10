/// interval { l: i32, r: i32 }
///
/// take a := interval, b := interval
///
/// 6 cases:
/// 1) intersection: b finishes after a
/// 2) intersection: a finishes after b
/// 3) intersection: b encloses a
/// 4) intersection: a encloses b
/// 5) no intersection: a does not intersect with b
/// 6) no intersection: b does not intersect with a
///
/// there are two cases per case as it is not known which number is greater
///
/// all of the cases are mutually exclusive, except for the last two in case of
/// the last two, those are mutually inclusive - one is impossible not
/// satisfying the other
///
/// this method allows to pick the solutions but is not fully correct - there is
/// an issue
///
/// lets see an example
///
/// first_list [[0, 10]]
/// second_list [[2, 3], [5, 7]]
///
/// there will be two intersections however using zip it will only pick one up
///
/// thus, another way of iterating should be used, where `a` and `b` should be
/// swapped on every increment of index if it gets hit
///
/// after getting hit, the index can be updated to be either a[1] or b[1], but
/// not greater than the current top index (see excalidraw)
pub struct Solution;

impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        // take a := interval, b := interval
        let mut first_list_index: usize = 0;
        let mut second_list_index: usize = 0;
        // let mut a: &Vec<i32>;
        // let mut b: &Vec<i32>;
        loop {
            if second_list_index == second_list.len() || first_list_index == first_list.len() {
                break;
            }
            let (a, b) = (
                &first_list[first_list_index],
                &second_list[second_list_index],
            );

            // 1) intersection: b finishes after a => b[0], a[1]
            if a[0] <= b[0] && b[0] <= a[1] && a[1] <= b[1] {
                res.push(vec![b[0], a[1]]);
                // push to next a
                first_list_index += 1;
                continue;
            }

            // 2) intersection: a finishes after b <-> b => a[0], b[1]
            if b[0] <= a[0] && a[0] <= b[1] && b[1] <= a[1] {
                res.push(vec![a[0], b[1]]);
                // push to next b
                second_list_index += 1;
                continue;
            }

            // 3) intersection: b encloses a => a[0], a[1]
            if a[0] >= b[0] && a[1] <= b[1] {
                res.push(vec![a[0], a[1]]);
                // push to next b
                second_list_index += 1;
                continue;
            }

            // 4) intersection: a encloses b => b[0], b[1]
            if b[0] >= a[0] && b[1] <= a[1] {
                res.push(vec![b[0], b[1]]);
                // push to next a
                first_list_index += 1;
                continue;
            }
            // 5), 6) no intersection => do nothing, increment smaller
            if a[0] <= b[0] {
                first_list_index += 1;
            } else {
                second_list_index += 1;
            }
        }

        res
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

    #[test]
    fn it_works_for_various_input_lengths() {
        let first_list = [[3, 5], [9, 20]].map(|sublist| sublist.to_vec()).to_vec();
        let second_list = [[4, 5], [7, 10], [11, 12], [14, 15], [16, 20]]
            .map(|sublist| sublist.to_vec())
            .to_vec();
        let result = Solution::interval_intersection(first_list, second_list);
        let want = [[4, 5], [9, 10], [11, 12], [14, 15], [16, 20]]
            .map(|sublist| sublist.to_vec())
            .to_vec();
        assert_eq!(result, want);
    }
}
