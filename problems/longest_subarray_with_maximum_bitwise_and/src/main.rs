struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut max_val = nums[0];
        let mut max_len = 1;
        let mut current_len = 1;

        for &num in nums.iter().skip(1) {
            if num > max_val {
                max_val = num;
                max_len = 1;
                current_len = 1;
            } else if num == max_val {
                current_len += 1;
                max_len = max_len.max(current_len);
            } else {
                current_len = 0;
            }
        }

        max_len
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
