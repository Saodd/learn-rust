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
        return inorder_traversal_recursion(&root, vec![]);
    }
}

fn inorder_traversal_recursion(
    root: &Option<Rc<RefCell<TreeNode>>>,
    mut result: Vec<i32>,
) -> Vec<i32> {
    match root {
        None => {}
        Some(tree) => {
            let node = tree.borrow();
            result = inorder_traversal_recursion(&node.left, result);
            result.push(node.val);
            result = inorder_traversal_recursion(&node.right, result);
        }
    }

    return result;
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

    // #[test]
    // fn check_myself() {}
    //
    // #[test]
    // fn commit_failed() {}
}
