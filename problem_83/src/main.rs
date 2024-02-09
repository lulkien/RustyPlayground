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

fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    let mut cache = head.as_ref().unwrap().val;
    let mut new_head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(cache)));
    println!("Cache: {cache}");
    let mut new_index_node = new_head.as_mut().unwrap();

    let mut index_node = &head.as_ref().unwrap().next;
    while let Some(ref inner) = index_node {
        if inner.val != cache {
            println!("Added: {}", inner.val);
            new_index_node.next = Some(Box::new(ListNode::new(inner.val)));
            cache = inner.val;
            new_index_node = new_index_node.next.as_mut().unwrap();
        }
        index_node = &inner.next;
    }

    new_head
}

fn main() {
    // Creating nodes
    let mut head = Some(Box::new(ListNode::new(1)));
    let mut node2 = Some(Box::new(ListNode::new(2)));
    let mut node3 = Some(Box::new(ListNode::new(2)));
    let mut node4 = Some(Box::new(ListNode::new(3)));
    let mut node5 = Some(Box::new(ListNode::new(3)));
    let mut node6 = Some(Box::new(ListNode::new(3)));
    let mut node7 = Some(Box::new(ListNode::new(4)));
    let mut node8 = Some(Box::new(ListNode::new(4)));
    let mut node9 = Some(Box::new(ListNode::new(4)));
    let node10 = Some(Box::new(ListNode::new(4)));

    // Linking nodes
    node9.as_mut().unwrap().next = node10;
    node8.as_mut().unwrap().next = node9;
    node7.as_mut().unwrap().next = node8;
    node6.as_mut().unwrap().next = node7;
    node5.as_mut().unwrap().next = node6;
    node4.as_mut().unwrap().next = node5;
    node3.as_mut().unwrap().next = node4;
    node2.as_mut().unwrap().next = node3;
    head.as_mut().unwrap().next = node2;

    // Printing the linked list
    let new_list = delete_duplicates(head);

    println!("-------------------------------");

    print_list(&new_list);
}

fn print_list(node: &Option<Box<ListNode>>) {
    if node.is_none() {
        println!("None");
        return;
    }

    let mut current = node;
    while let Some(ref inner_node) = *current {
        println!("Value: {}", inner_node.val);
        current = &inner_node.next;
    }
}
