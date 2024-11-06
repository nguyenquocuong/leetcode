struct Solution;

impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let mut last = -1;

        for bitlike in nums.chunk_by(|a, b| a.count_ones() == b.count_ones()) {
            let front = *bitlike.iter().min().unwrap();
            if front < last {
                return false;
            };
            last = *bitlike.iter().max().unwrap();
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
        assert!(Solution::can_sort_array(vec![8, 4, 2, 30, 15]));
    }

    #[test]
    fn testcase_2() {
        assert!(Solution::can_sort_array(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn testcase_3() {
        assert!(!Solution::can_sort_array(vec![3, 16, 8, 4, 2]));
    }
}
