struct Solution;

impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut s = s
            .chars()
            .map(|c| (c as u8 - b'a' + 1).to_string())
            .collect::<Vec<String>>()
            .join("");

        for _ in 0..k {
            s = s
                .chars()
                .map(|c| c as i32 - '0' as i32)
                .sum::<i32>()
                .to_string();
        }

        s.parse::<i32>().unwrap()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::get_lucky("iiii".to_string(), 1), 36);
        assert_eq!(Solution::get_lucky("leetcode".to_string(), 2), 6);
        assert_eq!(Solution::get_lucky("zbax".to_string(), 2), 8);
        assert_eq!(Solution::get_lucky("fleyctuuajsr".to_string(), 5), 8);
        assert_eq!(
            Solution::get_lucky("ituaazzknmlimmcqbejjgmaky".to_string(), 9),
            5
        );
    }
}
