struct Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        s.char_indices()
            .map(|(i, c)| match i {
                _
                    if s.len() >= 3 && i <= s.len() - 3 && s.as_bytes()[i + 1] == c as u8 &&
                           s.as_bytes()[i + 2] == c as u8 => "".into(),
                _ => c.to_string(),
            })
            .collect()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_fancy_string_1() {
        assert_eq!(
            Solution::make_fancy_string("leeetcode".to_string()),
            "leetcode"
        );
    }

    #[test]
    fn make_fancy_string_2() {
        assert_eq!(Solution::make_fancy_string("aaabaaaa".to_string()), "aabaa");
    }

    #[test]
    fn make_fancy_string_3() {
        assert_eq!(Solution::make_fancy_string("aab".to_string()), "aab");
    }

    #[test]
    fn make_fancy_string_4() {
        assert_eq!(Solution::make_fancy_string("a".to_string()), "a");
    }
}
