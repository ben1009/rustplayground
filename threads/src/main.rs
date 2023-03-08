fn main() {}

struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() {
            return l2;
        }
        if l2.is_none() {
            return l1;
        }

        let (mut l1, mut l2) = (l1, l2);
        let mut ret = Some(Box::new(ListNode::new(-1)));
        let mut head = &mut ret;
        let mut carry = 0;
        loop {
            if l1.is_none() && l2.is_none() && carry == 0 {
                break;
            }
            let v1 = match l1 {
                Some(n1) => {
                    l1 = n1.next;
                    n1.val
                }
                None => 0,
            };
            let v2 = match l2 {
                Some(n2) => {
                    l2 = n2.next;
                    n2.val
                }
                None => 0,
            };
            let mut num = v1 + v2 + carry;
            carry = num / 10;
            num %= 10;

            let next = ListNode::new(num);
            head.as_mut().unwrap().next = Some(Box::new(next));
            head = &mut head.as_mut().unwrap().next;
        }

        ret.unwrap().next
    }
}
