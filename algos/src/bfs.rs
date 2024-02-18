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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(max_depth_iteration(tree![3, 9, 20, null, null, 15, 7]), 3);
    }
}

pub fn to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::VecDeque;
    let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
    let mut queue = VecDeque::new();
    queue.push_back(head.as_ref().unwrap().clone());

    for children in vec[1..].chunks(2) {
        let parent = queue.pop_front().unwrap();
        if let Some(v) = children[0] {
            parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
            queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
        }
        if children.len() > 1 {
            if let Some(v) = children[1] {
                parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
            }
        }
    }
    head
}

#[macro_export]
macro_rules! tree {
    () => {
        None
    };
    ($($e:expr),*) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            to_tree(vec)
        }
    };
    ($($e:expr,)*) => {(tree![$($e),*])};

}
