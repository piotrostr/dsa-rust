pub fn length_of_longest_substring(_: String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let res = length_of_longest_substring("abcabcbb".to_string());
        assert_eq!(res, 3);
    }

    #[test]
    fn example_2() {
        let res = length_of_longest_substring("bbbb".to_string());
        assert_eq!(res, 1);
    }

    #[test]
    fn example_3() {
        let res = length_of_longest_substring("pwwkew".to_string());
        assert_eq!(res, 3);
    }
}
