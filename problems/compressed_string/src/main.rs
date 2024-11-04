struct Solution;

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut word = word;
        let mut comp = String::new();

        while !word.is_empty() {
            let mut count = 1;
            let mut chars = word.chars();

            let c = match chars.next().take() {
                Some(c) => c,
                None => return comp
            };

            while let Some(next) = chars.next().take() {
                match next.eq(&c) {
                    false => break,
                    true => match count >= 9 {
                        true => break,
                        false => count += 1
                    }
                }
            }

            comp.push_str(&count.to_string());
            comp.push(c);

            word = word[count..].to_string();
        }

        comp
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        assert_eq!(
            Solution::compressed_string("abcde".to_string()),
            "1a1b1c1d1e"
        );
    }

    #[test]
    fn testcase_2() {
        assert_eq!(
            Solution::compressed_string("aaaaaaaaaaaaaabb".to_string()),
            "9a5a2b"
        );
    }

    #[test]
    fn testcase_3() {
        assert_eq!(
            Solution::compressed_string("mrm".to_string()),
            "1m1r1m"
        );
    }
}
