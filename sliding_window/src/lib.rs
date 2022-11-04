pub fn length_of_longest_substring(s: String) -> i32 {
    let mut longest_substring = String::new();
    let mut max_length = 0;
    if s.is_empty() {
        return 0;
    }
    for c in s.chars() {
        longest_substring = longest_substring + &c.to_string();
        match validate(&longest_substring) {
            true => {
                if longest_substring.len() > max_length {
                    max_length = longest_substring.len();
                }
            }
            false => longest_substring = longest_substring[1..].to_string(),
        }
    }
    return max_length as i32;
}

pub fn validate(s: &String) -> bool {
    let mut charset = Vec::new();
    for c in s.chars() {
        match charset.contains(&c) {
            true => return false,
            false => charset.push(c),
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_works_for_invalid_strings() {
        let s = "asadf".to_string();
        assert_eq!(validate(&s), false)
    }

    #[test]
    fn validate_works_for_valid_strings() {
        let s = "asdfhgjl".to_string();
        assert_eq!(validate(&s), true)
    }

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
