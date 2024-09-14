struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        0
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_subarray_1() {
        assert_eq!(Solution::longest_subarray(vec![1, 2, 3, 3, 2, 2]), 2)
    }

    #[test]
    fn longest_subarray_2() {
        assert_eq!(Solution::longest_subarray(vec![1, 2, 3, 4]), 1)
    }
}
