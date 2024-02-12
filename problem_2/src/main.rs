// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut res_head: Option<Box<ListNode>> = None;
    let mut current_res = &mut res_head;
    let mut carry = 0;

    let mut current_1 = l1;
    let mut current_2 = l2;
    loop {
        let mut first = 0;
        let mut second = 0;
        if let Some(inner_node) = current_1 {
            first = inner_node.val;
            current_1 = inner_node.next;
        }

        if let Some(inner_node) = current_2 {
            second = inner_node.val;
            current_2 = inner_node.next;
        }

        let mut result = first + second + carry;
        if result >= 10 {
            result -= 10;
            carry = 1;
        } else {
            carry = 0;
        }

        *current_res = Some(Box::new(ListNode::new(result)));
        current_res = &mut current_res.as_mut().unwrap().next;


        if current_1.is_none() && current_2.is_none() {
            if carry == 1 {
                *current_res = Some(Box::new(ListNode::new(carry)));
            }
            break;
        }
    }

    res_head
}

fn main() {

}
