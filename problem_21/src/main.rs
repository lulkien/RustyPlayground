pub fn merge_two_lists(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
    let mut result = &mut l1;
    while l2.is_some() {
        if result.is_none() || l2.as_ref()?.val < result.as_ref()?.val {
            std::mem::swap(result, &mut l2);
        }
        result = &mut result.as_mut()?.next;
    }

    l1
}

fn main() {
    println!("Hello, world!");
}
