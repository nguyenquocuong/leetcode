struct Solution;

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut count = 0;
        for i in (0..s.len()).step_by(2) {
            if s.as_bytes()[i] != s.as_bytes()[i + 1] {
                count += 1;
            }
        }
        count
    }
}

fn main() {
    Solution::min_changes("1001".into());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        assert_eq!(Solution::min_changes("1001".into()), 2);
    }

    #[test]
    fn testcase_2() {
        assert_eq!(Solution::min_changes("10".into()), 1);
    }

    #[test]
    fn testcase_3() {
        assert_eq!(Solution::min_changes("0000".into()), 0);
    }
}
