use leetcode::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {}

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    pub fn dfs(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if left == None && right == None {
            return true;
        }
        if left == None || right == None {
            return false;
        }

        let left = left.unwrap();
        let right = right.unwrap();
        if left.borrow().val != right.borrow().val {
            return false;
        }
        return dfs(left.borrow().left.clone(), right.borrow().right.clone())
            && dfs(left.borrow().right.clone(), right.borrow().left.clone());
    }
    if root == None {
        return true;
    }
    let root = root.unwrap();
    return dfs(root.borrow().left.clone(), root.borrow().right.clone());
}
