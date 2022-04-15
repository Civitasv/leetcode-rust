use leetcode::tree::TreeNode;
use std::{cell::RefCell, rc::Rc};
// Definition for a binary tree node.
fn main() {
    let node = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    println!("Result: {:?}", inorder_traversal(node));
}

fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn helper(res: &mut Vec<i32>, root: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            helper(res, node.borrow_mut().left.take());
            res.push(node.borrow().val);
            helper(res, node.borrow_mut().right.take());
        }
    }
    let mut res = vec![];
    helper(&mut res, root);

    return res;
}
