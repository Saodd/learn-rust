use super::helper::list_node::ListNode;
use super::Solution;

impl Solution {
    /// 21. 合并两个有序链表（简单）
    ///
    /// 将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。
    ///
    /// 提示：
    ///
    /// 两个链表的节点数目范围是 `[0, 50]`  
    /// -100 <= Node.val <= 100  
    /// l1 和 l2 均按 非递减顺序 排列  
    ///
    /// 来源：力扣（LeetCode）  
    /// 链接：<https://leetcode.cn/problems/merge-two-sorted-lists/>  
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        return match (list1, list2) {
            (None, None) => None,
            (None, l2) => l2,
            (l1, None) => l1,
            (Some(mut l1), Some(mut l2)) => {
                if l1.val < l2.val {
                    l1.next = Self::merge_two_lists(l1.next, Some(l2));
                    Some(l1)
                } else {
                    l2.next = Self::merge_two_lists(Some(l1), l2.next);
                    Some(l2)
                }
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        {
            let list1 = ListNode::new_from_vec(vec![1, 2, 4]);
            let list2 = ListNode::new_from_vec(vec![1, 3, 4]);
            let res = Solution::merge_two_lists(list1, list2);
            let answer = ListNode::new_from_vec(vec![1, 1, 2, 3, 4, 4]);
            assert_eq!(res, answer);
        }
        {
            let list1 = ListNode::new_from_vec(vec![]);
            let list2 = ListNode::new_from_vec(vec![]);
            let res = Solution::merge_two_lists(list1, list2);
            let answer = ListNode::new_from_vec(vec![]);
            assert_eq!(res, answer);
        }
        {
            let list1 = ListNode::new_from_vec(vec![]);
            let list2 = ListNode::new_from_vec(vec![0]);
            let res = Solution::merge_two_lists(list1, list2);
            let answer = ListNode::new_from_vec(vec![0]);
            assert_eq!(res, answer);
        }
    }

    // #[test]
    // fn check_myself() {}
    //
    // #[test]
    // fn commit_failed() {}
}
