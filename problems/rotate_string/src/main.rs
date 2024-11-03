struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        match s.eq(&goal) {
            true => true,
            false => {
                let chars: Vec<char> = s.chars().collect();
                let n = chars.len();

                for i in 0..n - 1 {
                    if chars[i + 1..]
                        .iter()
                        .chain(chars[..=i].iter())
                        .collect::<String>()
                        .eq(&goal)
                    {
                        return true;
                    }
                }

                false
            }
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        assert!(Solution::rotate_string("abcde".into(), "cdeab".into()))
    }

    #[test]
    fn testcase_2() {
        assert!(!Solution::rotate_string("abcde".into(), "abced".into()))
    }

    #[test]
    fn testcase_3() {
        assert!(Solution::rotate_string("abcde".into(), "eabcd".into()))
    }

    #[test]
    fn testcase_4() {
        assert!(Solution::rotate_string("abcde".into(), "abcde".into()))
    }
}
