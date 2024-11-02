struct Solution;

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let words: Vec<&str> = sentence.split(" ").collect();
        let n = words.len();

        if words[0].as_bytes().first().unwrap() != words[n - 1].as_bytes().last().unwrap() {
            return false;
        }

        for i in 0..n - 1 {
            if words[i].as_bytes().last().unwrap() != words[i + 1].as_bytes().first().unwrap() {
                return false;
            }
        }

        true
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        assert!(Solution::is_circular_sentence(
            "leetcode exercises sound delightful".into()
        ))
    }

    #[test]
    fn testcase_2() {
        assert!(Solution::is_circular_sentence("eetcode".into()))
    }

    #[test]
    fn testcase_3() {
        assert!(!Solution::is_circular_sentence("Leetcode is cool".into()))
    }

    #[test]
    fn testcase_4() {
        assert!(!Solution::is_circular_sentence("leetcode".into()))
    }
}
