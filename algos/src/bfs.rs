// Breath First Search
// 非递归做法是采用队列,广度优先遍只有迭代法

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
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

// 迭代 & 层序遍历bfs & stack 不是应该用QUEUE？
#[allow(dead_code)]
pub fn max_depth_iteration(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let mut max_depth: i32 = 0;
    let mut stack = vec![root.unwrap()];
    while !stack.is_empty() {
        let num = stack.len();
        for _i in 0..num {
            let top = stack.remove(0);
            if top.borrow_mut().left.is_some() {
                stack.push(top.borrow_mut().left.take().unwrap());
            }
            if top.borrow_mut().right.is_some() {
                stack.push(top.borrow_mut().right.take().unwrap());
            }
        }
        max_depth += 1;
    }
    max_depth
}
#[allow(dead_code)]
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let mut queue: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    queue.push(root.unwrap());
    let mut last_level_count = 1;
    let mut this_level_count = 0;
    let mut levels = 0;

    while !queue.is_empty() {
        let last_node: Rc<RefCell<TreeNode>> = queue.remove(0);
        if last_node.as_ref().borrow().left.is_some() {
            this_level_count += 1;
            queue.push(last_node.as_ref().borrow().left.clone().unwrap());
        }
        if last_node.as_ref().borrow().right.is_some() {
            this_level_count += 1;
            queue.push(last_node.as_ref().borrow().right.clone().unwrap());
        }
        last_level_count -= 1;
        if last_level_count == 0 {
            levels += 1;
            last_level_count = this_level_count;
            this_level_count = 0;
        }
    }

    return levels;
}

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut queue = VecDeque::new();
    if root.is_some() {
        queue.push_back(root);
    }
    while !queue.is_empty() {
        let mut temp = vec![];
        for _ in 0..queue.len() {
            let node = queue.pop_front().unwrap().unwrap();
            temp.push(node.borrow().val);
            if node.borrow().left.is_some() {
                queue.push_back(node.borrow().left.clone());
            }
            if node.borrow().right.is_some() {
                queue.push_back(node.borrow().right.clone());
            }
        }
        res.push(temp);
    }
    res
}
pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut queue = VecDeque::new();
    if root.is_some() {
        queue.push_back(root);
    }
    while !queue.is_empty() {
        let mut temp = vec![];
        for _ in 0..queue.len() {
            let node = queue.pop_front().unwrap().unwrap();
            temp.push(node.borrow().val);
            if node.borrow().left.is_some() {
                queue.push_back(node.borrow().left.clone());
            }
            if node.borrow().right.is_some() {
                queue.push_back(node.borrow().right.clone());
            }
        }
        res.push(temp);
    }
    res.into_iter().rev().collect()
}

// pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//     if root.is_none(){
//         return 0;
//     }
//     let mut res = 0;
//     let mut queue = LinkedList::new();
//     queue.push_back(root.clone());
//     let mut qsize = queue.len();
//     while queue.len()>0 {
//         for i in 0..qsize {
//             let cmp = queue.pop_front();
//             if let Some(Some(rced_node)) = cmp {
//                 let node = rced_node.borrow();
//                 if !node.left.is_none() {
//                     queue.push_back(node.left.clone());
//                 }
//                 if !node.right.is_none() {
//                     queue.push_back(node.right.clone());
//                 }

//             }
//         }
//         qsize=queue.len();
//         res += 1;
//     }
//     res

// }

