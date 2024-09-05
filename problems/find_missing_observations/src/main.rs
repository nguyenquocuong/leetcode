struct Solution;

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let total_rolls = rolls.len() as i32 + n;
        let m_sum: i32 = rolls.iter().sum();
        let n_sum = (mean * total_rolls) - m_sum;
        println!("{}", n_sum);

        if n_sum <= 6 * n {
            let mut missing_rolls = vec![n_sum / n; n as usize];
            missing_rolls
        } else {
            vec![]
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        assert_eq!(Solution::missing_rolls(vec![3, 2, 4, 3], 4, 2), vec![6, 6]);
    }

    #[test]
    fn testcase_2() {
        assert_eq!(
            Solution::missing_rolls(vec![1, 5, 6], 3, 4),
            vec![2, 3, 2, 2]
        );
    }

    #[test]
    fn testcase_3() {
        assert_eq!(Solution::missing_rolls(vec![1, 2, 3, 4], 6, 4), vec![]);
    }

    #[test]
    fn testcase_4() {
        assert_eq!(
            Solution::missing_rolls(vec![3, 2, 4, 3], 4, 3),
            vec![6, 5, 5]
        );
    }

    #[test]
    fn testcase_5() {
        assert_eq!(
            Solution::missing_rolls(vec![3, 2, 4, 3], 2, 5),
            vec![1, 1, 1, 2, 1]
        );
    }
}
