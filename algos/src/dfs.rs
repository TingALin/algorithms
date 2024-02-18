// Depth-First-Search
// 非递归做法是采用栈
// 前序遍历 (preorder traversal) - 中序遍历 (inorder traversal) - 后序遍历 (postorder traversal)
// https://leetcode.cn/problems/maximum-depth-of-binary-tree/solutions/1797307/by-carlsun-2-ojzh/

use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::max;

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

// 深度优先
// 递归计算出其左子树和右子树的最大深度，给出二叉树的最大深度
// 时间复杂度：O(n),其中 n 为二叉树节点的个数。每个节点在递归中只被遍历一次
// 空间复杂度：O(maxHeight)，其中 maxHeight 表示二叉树的高度。递归函数需要栈空间，而栈空间取决于递归的深度，因此空间复杂度等价于二叉树的高度
// 后序遍历（左右中）（自下而上）, 求的是高度
#[allow(dead_code)]
pub fn max_depth_recusion(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_val = 0;
    if let Some(node) = root {
        let left = 1 + max_depth_recusion(node.borrow_mut().left.take());
        let right = 1 + max_depth_recusion(node.borrow_mut().right.take());
        max_val = max(left, right);
    }
    max_val

    // match root {
    //     Some(node) => {
    //         let left = max_depth(node.borrow().left.clone());
    //         let right = max_depth(node.borrow().right.clone());
    //         println!("{:?}", right);
    //         1 + left.max(right)
    //     }
    //     _ => 0,
    // }
}

// 前序遍历(自上而下)（中左右）
// function maxDepth(root: TreeNode | null): number {
//     function recur(node: TreeNode | null, count: number) {
//         if (node === null) {
//             resMax = resMax > count ? resMax : count;
//             return;
//         }
//         recur(node.left, count + 1);
//         recur(node.right, count + 1);
//     }
//     let resMax: number = 0;
//     let count: number = 0;
//     recur(root, count);
//     return resMax;
// };


// 迭代 & 层序遍历bfs & stack
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
mod dfs_tests {
    use super::*;

    #[test]
    fn max_depth_recusion_test() {
        let list = Some(Rc::new(RefCell::new(TreeNode{ val:3, left: Some(Rc::new(RefCell::new(TreeNode::new(9)))), right: Some(Rc::new(RefCell::new(TreeNode{val: 20, left: Some(Rc::new(RefCell::new(TreeNode::new(15)))), right: Some(Rc::new(RefCell::new(TreeNode::new(7)))) }))) })));

        assert_eq!(max_depth_recusion(list), 3);
    }
}