struct Solution;

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut output: Vec<i32> = vec![0; queries.len()];

        for (i, query) in queries.iter().enumerate() {
            let mut acc = arr[query[0] as usize];
            for j in query[0] + 1..=query[1] {
                acc ^= arr[j as usize]
            }
            output[i] = acc;
        }

        output
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_queries_1() {
        assert_eq!(
            Solution::xor_queries(
                vec![1, 3, 4, 8],
                vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]]
            ),
            vec![2, 7, 14, 8]
        );
    }

    #[test]
    fn xor_queries_2() {
        assert_eq!(
            Solution::xor_queries(
                vec![4, 8, 2, 10],
                vec![vec![2, 3], vec![1, 3], vec![0, 0], vec![0, 3]]
            ),
            vec![8, 0, 4, 4]
        );
    }
}
