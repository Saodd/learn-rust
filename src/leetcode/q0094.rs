use std::cell::RefCell;
use std::rc::Rc;

use super::helper::tree_node::TreeNode;
use super::Solution;

impl Solution {
    /// 94. 二叉树的中序遍历（简单）
    ///
    /// 给定一个二叉树的根节点 root ，返回 它的 **中序** 遍历 。
    ///
    /// 提示：  
    /// 树中节点数目在范围 `[0, 100]` 内  
    /// -100 <= Node.val <= 100
    ///
    /// 来源：力扣（LeetCode）  
    /// 链接：<https://leetcode.cn/problems/binary-tree-inorder-traversal/>
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, mut result: Vec<i32>) -> Vec<i32> {
            match root {
                None => {}
                Some(tree) => {
                    let node = tree.borrow();
                    result = recurse(&node.left, result);
                    result.push(node.val);
                    result = recurse(&node.right, result);
                }
            }

            return result;
        }

        return recurse(&root, vec![]);
    }

    /// 94. 二叉树的中序遍历（简单）
    ///
    /// 进阶: 递归算法很简单，你可以通过迭代算法完成吗？
    pub fn inorder_traversal_v2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        enum Mark {
            Todo(Rc<RefCell<TreeNode>>),
            Val(i32),
        }

        let mut res = vec![];
        let mut stack = vec![];
        if let Some(root) = root {
            stack.push(Mark::Todo(root))
        }

        loop {
            match stack.pop() {
                None => break,
                Some(mark) => match mark {
                    Mark::Val(val) => res.push(val),
                    Mark::Todo(node) => {
                        let borrowed = node.borrow();

                        if let Some(right) = &borrowed.right {
                            stack.push(Mark::Todo(right.clone()))
                        }
                        stack.push(Mark::Val(borrowed.val));
                        if let Some(left) = &borrowed.left {
                            stack.push(Mark::Todo(left.clone()))
                        }
                    }
                },
            }
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        {
            let tree = TreeNode::new_from_vec(&vec![1, 0, 2, 0, 0, 3]);
            let result = Solution::inorder_traversal(tree);
            let correct = vec![1, 3, 2];
            assert_eq!(result, correct);
        }
        {
            let tree = TreeNode::new_from_vec(&vec![]);
            let result = Solution::inorder_traversal(tree);
            let correct = vec![];
            assert_eq!(result, correct);
        }
        {
            let tree = TreeNode::new_from_vec(&vec![1]);
            let result = Solution::inorder_traversal(tree);
            let correct = vec![1];
            assert_eq!(result, correct);
        }
    }

    #[test]
    fn examples_v2() {
        {
            let tree = TreeNode::new_from_vec(&vec![1, 0, 2, 0, 0, 3]);
            let result = Solution::inorder_traversal_v2(tree);
            let correct = vec![1, 3, 2];
            assert_eq!(result, correct);
        }
        {
            let tree = TreeNode::new_from_vec(&vec![]);
            let result = Solution::inorder_traversal_v2(tree);
            let correct = vec![];
            assert_eq!(result, correct);
        }
        {
            let tree = TreeNode::new_from_vec(&vec![1]);
            let result = Solution::inorder_traversal_v2(tree);
            let correct = vec![1];
            assert_eq!(result, correct);
        }
    }

    // #[test]
    // fn check_myself() {}
    //
    // #[test]
    // fn commit_failed() {}
}
