use leetcode::linkedlist::ListNode;

fn main() {}
fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut slow, mut fast) = (&head, &head);

    while fast.as_ref() != None && fast.as_ref()?.next != None {
        slow = &slow.as_ref()?.next;
        fast = &fast.as_ref()?.next.as_ref()?.next;
    }
    return slow.clone();
}
