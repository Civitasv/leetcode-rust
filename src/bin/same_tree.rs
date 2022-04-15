use std::{cell::RefCell, rc::Rc};

use leetcode::tree::TreeNode;

fn main() {}

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn helper(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (Some(a), Some(b)) => {
                a.borrow().val == b.borrow().val
                    && helper(&a.borrow().left, &b.borrow().left)
                    && helper(&a.borrow().right, &b.borrow().right)
            }
            (None, None) => true,
            (Some(_), None) => false,
            (None, Some(_)) => false,
        }
    }
    helper(&p, &q)
}
