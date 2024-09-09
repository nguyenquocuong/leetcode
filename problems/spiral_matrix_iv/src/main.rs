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
        //println!("{head:?}");
        vec![]
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
}
