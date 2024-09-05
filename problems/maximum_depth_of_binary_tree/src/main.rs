struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(root) => {
                1 + (Self::max_depth(root.borrow().left.clone()))
                    .max(Self::max_depth(root.borrow().right.clone()))
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
            Solution::max_depth(Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    ..TreeNode::default()
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 20,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 15,
                        ..TreeNode::default()
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        ..TreeNode::default()
                    })))
                })))
            })))),
            3
        );
    }

    #[test]
    fn testcase_2() {
        assert_eq!(
            Solution::max_depth(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    ..TreeNode::default()
                })))
            })))),
            2
        );
    }
}
