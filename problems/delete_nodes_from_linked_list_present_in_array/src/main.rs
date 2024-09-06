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
}

impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::modified_list_recursive(std::collections::HashSet::from_iter(nums), head)
    }

    pub fn modified_list_recursive(
        nums: std::collections::HashSet<i32>,
        head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(mut node) => {
                if nums.contains(&node.val) {
                    Solution::modified_list_recursive(nums, node.next)
                } else {
                    node.next = Solution::modified_list_recursive(nums, node.next);
                    Some(node)
                }
            }
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
            Solution::modified_list(
                vec![1, 2, 3],
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode { val: 5, next: None }))
                            }))
                        }))
                    }))
                }))
            ),
            Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 5, next: None }))
            }))
        )
    }

    #[test]
    fn testcase_2() {
        assert_eq!(
            Solution::modified_list(
                vec![1],
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 1,
                            next: Some(Box::new(ListNode {
                                val: 2,
                                next: Some(Box::new(ListNode {
                                    val: 1,
                                    next: Some(Box::new(ListNode { val: 2, next: None }))
                                }))
                            }))
                        }))
                    }))
                }))
            ),
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 2, next: None }))
                }))
            }))
        )
    }

    #[test]
    fn testcase_3() {
        assert_eq!(
            Solution::modified_list(
                vec![5],
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode { val: 4, next: None }))
                        }))
                    }))
                }))
            ),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 4, next: None }))
                    }))
                }))
            }))
        )
    }
}
