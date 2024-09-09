use std::ops::Add;

struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from_vec(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut current = &mut head;

        for val in values {
            let node = Box::new(ListNode::new(val));
            current.replace(node);
            if let Some(node) = current {
                current = &mut node.next;
            }
        }

        head
    }
}

impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let mut output: Vec<Vec<i32>> = vec![vec![-1; n as usize]; m as usize];
        let mut direction: (i32, i32) = (0, 1);
        let mut position: (i32, i32) = (0, 0);
        let mut current = head;

        while let Some(node) = current {
            let next = (position.0 + direction.0, position.1 + direction.1);
            if next.0 < 0
                || next.0 == m
                || next.1 < 0
                || next.1 == n
                || output[next.0 as usize][next.1 as usize] != -1
            {
                direction = match direction {
                    (0, 1) => (1, 0),
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    (-1, 0) => (0, 1),
                    _ => unreachable!(),
                };
            }

            output[position.0 as usize][position.1 as usize] = node.val;
            position.0 += direction.0;
            position.1 += direction.1;
            current = node.next;
        }

        output
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        assert_eq!(
            Solution::spiral_matrix(
                3,
                5,
                ListNode::from_vec(vec![3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0])
            ),
            vec![
                vec![3, 0, 2, 6, 8],
                vec![5, 0, -1, -1, 1],
                vec![5, 2, 4, 9, 7]
            ]
        )
    }

    #[test]
    fn testcase_2() {
        assert_eq!(
            Solution::spiral_matrix(1, 4, ListNode::from_vec(vec![0, 1, 2])),
            vec![vec![0, 1, 2, -1]]
        )
    }
}
