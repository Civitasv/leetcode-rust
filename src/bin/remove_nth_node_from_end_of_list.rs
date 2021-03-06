use leetcode::linkedlist::ListNode;

fn main() {}

fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    dummy.next = head;
    let mut dummy = Box::new(dummy);
    let mut fast = dummy.clone();
    let mut slow = dummy.as_mut();
    // move fast n forward
    for _ in 0..n {
        fast = fast.next.unwrap();
    }

    while fast.next.is_some() {
        fast = fast.next.unwrap();
        slow = slow.next.as_mut().unwrap();
    }
    let next = slow.next.as_mut().unwrap();
    slow.next = next.next.clone();
    dummy.next
}
