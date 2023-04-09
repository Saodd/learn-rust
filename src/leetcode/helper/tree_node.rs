use std::cell::RefCell;
use std::rc::Rc;

/// 典型的二叉树
#[derive(Debug, PartialEq, Eq)]
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

    /// 二叉树可以用数组来表示。
    /// 为了简化，`nums`中的数字`0`表示`null`。
    pub fn new_from_vec(nums: &Vec<i32>) -> Option<Rc<RefCell<Self>>> {
        return Self::_new_from_vec(nums, 1);
    }

    /// new_from_vec 所使用的迭代函数。
    /// `pos`表示在『根节点从1开始计数的虚拟二叉树』中对应的位置编号，对应实际数组下标`index==pos-1`。
    /// 为了简化，`nums`中的数字`0`表示`null`。
    fn _new_from_vec(nums: &Vec<i32>, pos: usize) -> Option<Rc<RefCell<Self>>> {
        return match pos <= nums.len() {
            true => match nums[pos - 1] {
                0 => None,
                val => Some(Rc::new(RefCell::new(Self {
                    val,
                    left: Self::_new_from_vec(nums, pos * 2),
                    right: Self::_new_from_vec(nums, pos * 2 + 1),
                }))),
            },
            false => None,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_from_vec() {
        {
            let tree1 = TreeNode::new_from_vec(&vec![]);
            let tree2 = None;
            assert_eq!(tree1, tree2);
        }
        {
            let tree1 = TreeNode::new_from_vec(&vec![1]);
            let tree2 = Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            })));
            assert_eq!(tree1, tree2);
        }
        {
            let tree1 = TreeNode::new_from_vec(&vec![1, 0, 2, 0, 0, 3]);
            let tree2 = Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
            })));
            assert_eq!(tree1, tree2);
        }
    }
}
