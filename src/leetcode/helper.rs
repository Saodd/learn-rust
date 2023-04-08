#[derive(PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn new_from_vec(nums: Vec<i32>) -> Option<Box<Self>> {
        let mut head: Box<Self> = Box::new(Self::new(0));

        for num in nums.iter().rev() {
            head.val = *num;
            head = Box::new(ListNode {
                next: Some(head),
                val: 0,
            })
        }
        return head.next;
    }
}

impl std::fmt::Debug for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = format!("ListNode{{{}", self.val);

        let mut cur = &self.next;
        // 注：因为ListNode拥有next的所有权，因此不可能产生循环引用问题
        loop {
            match cur {
                None => break,
                Some(list) => {
                    res.push_str(&format!(" -> {}", list.val));
                    cur = &list.next;
                }
            }
        }

        res.push('}');
        return f.write_str(&res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_from_vec() {
        // 0
        assert_eq!(ListNode::new_from_vec(vec![]), None);

        // 1
        let list1 = Box::new(ListNode::new(123));
        let list2 = ListNode::new_from_vec(vec![123]).unwrap();
        assert_eq!(list1, list2);

        // 2
        let list1 = Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        });
        let list2 = ListNode::new_from_vec(vec![1, 2]).unwrap();
        assert_eq!(list1, list2);
        let list2 = ListNode::new_from_vec(vec![1, 3]).unwrap();
        assert_ne!(list1, list2);
    }

    #[test]
    fn debug() {
        let list = ListNode::new(0);
        assert_eq!(format!("{:?}", list), "ListNode{0}");

        let list = ListNode::new_from_vec(vec![1, 2]);
        assert_eq!(format!("{:?}", list), "Some(ListNode{1 -> 2})");
    }
}
