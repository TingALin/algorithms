// Breath First Search
// 非递归做法是采用队列

use std::cell::RefCell;
use std::rc::Rc;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// 迭代 & 层序遍历bfs & stack 不是应该用QUEUE？ 有前/中/后序
#[allow(dead_code)]
pub fn max_depth_iteration(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none(){
        return 0;
    }
    let mut max_depth: i32 = 0;
    let mut stack = vec![root.unwrap()];
    while !stack.is_empty() {
        let num = stack.len();
        for _i in 0..num{
            let top = stack.remove(0);
            if top.borrow_mut().left.is_some(){
                stack.push(top.borrow_mut().left.take().unwrap());
            }
            if top.borrow_mut().right.is_some(){
                stack.push(top.borrow_mut().right.take().unwrap());
            }
            }
        max_depth+=1;
        }
    max_depth
}
