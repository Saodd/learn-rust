use super::helper::ListNode;
use super::Solution;

impl Solution {
    /// 21. 合并两个有序链表
    ///
    /// 将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。
    ///
    /// 提示：
    ///
    /// 两个链表的节点数目范围是 `[0, 50]`
    /// -100 <= Node.val <= 100
    /// l1 和 l2 均按 非递减顺序 排列
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = list1;
        let mut l2 = list2;

        let mut dumb = Box::new(ListNode::new(0));
        let mut tail = &mut dumb;
        loop {
            let temp1 = match &mut l1 {
                None => {
                    tail.next = l2;
                    break;
                }
                Some(list) => list,
            };
            let temp2 = match &mut l2 {
                None => {
                    tail.next = l1;
                    break;
                }
                Some(list) => list,
            };
            if temp1.val < temp2.val {
                let next = temp1.next.take();
                tail.next = l1;
                l1 = next;
            } else {
                let next = temp2.next.take();
                tail.next = l2;
                l2 = next;
            }
            tail = tail.next.as_mut().unwrap();
        }

        return dumb.next;
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
