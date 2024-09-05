struct Solution;

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let n_sum = mean * (rolls.len() as i32 + n) - rolls.iter().sum::<i32>();

        if n_sum >= n && n_sum <= 6 * n {
            let mut missing_rolls = vec![n_sum / n; n as usize];
            let mut remaning = n_sum - (n_sum / n) * n;
            let mut i = 0;

            loop {
                if remaning == 0 {
                    break;
                }

                missing_rolls[i] += 1;
                remaning -= 1;
                i += 1;
            }
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
        assert_eq!(
            Solution::missing_rolls(vec![3, 2, 4, 3], 4, 2)
                .iter()
                .sum::<i32>(),
            [6, 6].iter().sum()
        );
    }

    #[test]
    fn testcase_2() {
        assert_eq!(
            Solution::missing_rolls(vec![1, 5, 6], 3, 4)
                .iter()
                .sum::<i32>(),
            [2, 3, 2, 2].iter().sum()
        );
    }

    #[test]
    fn testcase_3() {
        assert_eq!(
            Solution::missing_rolls(vec![1, 2, 3, 4], 6, 4)
                .iter()
                .sum::<i32>(),
            [].iter().sum::<i32>()
        );
    }

    #[test]
    fn testcase_4() {
        assert_eq!(
            Solution::missing_rolls(vec![3, 2, 4, 3], 4, 3)
                .iter()
                .sum::<i32>(),
            [6, 5, 5].iter().sum()
        );
    }

    #[test]
    fn testcase_5() {
        assert_eq!(
            Solution::missing_rolls(vec![3, 2, 4, 3], 2, 5)
                .iter()
                .sum::<i32>(),
            [1, 1, 1, 2, 1].iter().sum()
        );
    }

    #[test]
    fn testcase_6() {
        assert_eq!(
            Solution::missing_rolls(vec![6, 3, 4, 3, 5, 3], 1, 6)
                .iter()
                .sum::<i32>(),
            [].iter().sum()
        );
    }

    #[test]
    fn testcase_7() {
        assert_eq!(
            Solution::missing_rolls(
                vec![
                    4, 2, 2, 5, 4, 5, 4, 5, 3, 3, 6, 1, 2, 4, 2, 1, 6, 5, 4, 2, 3, 4, 2, 3, 3, 5,
                    4, 1, 4, 4, 5, 3, 6, 1, 5, 2, 3, 3, 6, 1, 6, 4, 1, 3
                ],
                2,
                53
            )
            .iter()
            .sum::<i32>(),
            [].iter().sum()
        );
    }
}
