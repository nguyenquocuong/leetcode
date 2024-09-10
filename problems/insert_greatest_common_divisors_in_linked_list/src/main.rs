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
    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = head;
        let mut current = dummy.as_mut();
        while let Some(node) = current {
            if let Some(next_node) = node.next.take() {
                let pivot = Box::new(ListNode {
                    val: Solution::gcd(node.val, next_node.val),
                    next: Some(next_node),
                });
                node.next = Some(pivot);
            }
            current = node.next.as_mut().and_then(|next| next.next.as_mut());
        }
        dummy
    }

    pub fn gcd(a: i32, b: i32) -> i32 {
        let mut result = a.min(b);
        while result > 0 {
            if a % result == 0 && b % result == 0 {
                break;
            }
            result -= 1;
        }
        result
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_1() {
        assert_eq!(Solution::gcd(18, 6), 6);
        assert_eq!(Solution::gcd(6, 10), 2);
        assert_eq!(Solution::gcd(10, 3), 1);
    }

    #[test]
    fn testcase_1() {
        assert_eq!(
            Solution::insert_greatest_common_divisors(ListNode::from_vec(vec![18, 6, 10, 3])),
            ListNode::from_vec(vec![18, 6, 6, 2, 10, 1, 3])
        )
    }

    #[test]
    fn testcase_2() {
        assert_eq!(
            Solution::insert_greatest_common_divisors(ListNode::from_vec(vec![7])),
            ListNode::from_vec(vec![7])
        )
    }
}
