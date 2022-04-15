use std::{cell::RefCell, rc::Rc};
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TreeNode;

    use std::{cell::RefCell, rc::Rc};
    #[test]
    fn test_tree() {
        let node = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        assert_eq!(inorder_traversal(node), vec![2]);
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
}
