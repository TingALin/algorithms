use std::rc::Rc;
use std::cell::RefCell;

// https://leetcode.cn/problems/same-tree/description/
#[allow(dead_code)]
pub struct TreeNode {
    val:i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
impl TreeNode {
    pub fn new(val: i32) -> Self{
        TreeNode { val, left: None, right: None }
    }
}

// O(n)
#[allow(dead_code)]
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>)-> bool{
    match(p,q) {
        (Some(p), Some(q)) => {
            let pb = p.borrow();
            let qb = q.borrow();
            pb.val == qb.val && self::is_same_tree(pb.left.clone(), qb.left.clone()) 
                            && self::is_same_tree(pb.right.clone(), qb.right.clone())
        },
        (None, None) => true,
        _ => false,
    }
}

#[cfg(test)]
mod recusion_tests {
    use super::*;

    #[test]
    fn same_tree_bool() {
        let p = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: Some(Rc::new(RefCell::new(TreeNode { val: 2, left: None, right: None }))), right: Some(Rc::new(RefCell::new(TreeNode { val: 3, left: None, right: None })))})) );
        let q = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: Some(Rc::new(RefCell::new(TreeNode { val: 2, left: None, right: None }))), right: Some(Rc::new(RefCell::new(TreeNode { val: 3, left: None, right: None })))})) );

        assert_eq!(is_same_tree(p, q), true);
        //println!("Reference Count of rc_q: {}", Rc::strong_count(&q.unwrap()));
    }
}