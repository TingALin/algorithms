// Depth-First-Search
// 非递归做法是采用栈，深度优先遍历有递归法，迭代法
// 前序遍历 (preorder traversal) - 中序遍历 (inorder traversal) - 后序遍历 (postorder traversal)
// https://leetcode.cn/problems/maximum-depth-of-binary-tree/solutions/1797307/by-carlsun-2-ojzh/
// https://programmercarl.com/%E4%BA%8C%E5%8F%89%E6%A0%91%E7%9A%84%E8%BF%AD%E4%BB%A3%E9%81%8D%E5%8E%86.html#%E5%85%B6%E4%BB%96%E8%AF%AD%E8%A8%80%E7%89%88%E6%9C%AC

use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

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

#[allow(dead_code)]
// https://leetcode.cn/problems/binary-tree-preorder-traversal/solutions/1503889/custer-by-custerfun-l9zm/
fn preorder_traversal(nodes: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    if nodes.is_none() {
        return result;
    }
    let mut stack = Vec::new();
    let mut r = nodes.clone();

    while r.is_some() || !stack.is_empty() {
        while let Some(node) = r {
            result.push(node.borrow().val);
            stack.push(node.clone());
            r = node.borrow().left.clone();
        }
        r = stack.pop();
        if let Some(node) = r {
            r = node.borrow().right.clone();
        }
    }
    result
}

#[allow(dead_code)]
fn preorder_recursive(nodes: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];
    if nodes.is_none() {
        return result;
    }
    pre_recursive(nodes, &mut result);
    result
}
fn pre_recursive(nodes: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    match nodes {
        Some(node) => {
            result.push(node.borrow().val);
            // 递归遍历左子树
            pre_recursive(node.borrow().left.clone(), result);
            // 递归遍历右子树
            pre_recursive(node.borrow().right.clone(), result);
        }
        None => {}
    }
}

#[allow(dead_code)]
// https://leetcode.cn/problems/binary-tree-postorder-traversal/solutions/1503897/custer-by-custerfun-9gux/
fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];
    if root.is_none() {
        return result;
    }
    let mut stack1 = Vec::new();
    let mut stack2 = Vec::new();
    stack1.push(root);
    // 将 stack1 栈顶的节点依次出栈，并将改节点放入 stack2，将该节点的左右子节点入 stack1
    while let Some(Some(node)) = stack1.pop() {
        if node.borrow().left.is_some() {
            stack1.push(node.borrow().left.clone());
        }
        if node.borrow().right.is_some() {
            stack1.push(node.borrow().right.clone());
        }
        stack2.push(Some(node));
    }
    // 将 stack2 栈顶的节点依次出栈，并访问其节点值
    while let Some(Some(node)) = stack2.pop() {
        result.push(node.borrow().val);
    }
    result
}

#[allow(dead_code)]
// https://leetcode.cn/problems/binary-tree-inorder-traversal/solutions/1503890/er-cha-shu-by-custerfun-zxm3/
fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];
    if root.is_none() {
        return result;
    }

    // 使用栈来保存需要返回后处理的节点
    let mut stack = Vec::new();
    let mut r = root.clone();

    // 满足当前节点非空或者栈非空时执行循环
    while r.is_some() || !stack.is_empty() {
        // 若当前节点非空，将当前节点入栈，并进入其左子树访问
        while let Some(node) = r {
            stack.push(node.clone());
            r = node.borrow().left.clone();
        }

        // 栈顶的节点出栈，访问其节点值，并进入其右子树访问
        r = stack.pop();
        if let Some(node) = r {
            result.push(node.borrow().val);
            r = node.borrow().right.clone();
        }
    }

    result
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

#[cfg(test)]
mod dfs_tests {
    use crate::tree;

    use super::*;

    #[test]
    fn max_depth_recusion_test() {
        let list = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));

        assert_eq!(max_depth_recusion(list), 3);
    }

    #[test]
    fn preorder_traversal_test() {
        assert_eq!(preorder_traversal(tree!(1, null, 2, 3)), [1, 2, 3]);
    }

    #[test]
    fn postorder_traversal_test() {
        assert_eq!(postorder_traversal(tree!(1, null, 2, 3)), [3, 2, 1]);
    }

    #[test]
    fn inorder_traversal_test() {
        assert_eq!(inorder_traversal(tree!(1, null, 2, 3)), [1, 3, 2]);
    }
}

#[allow(dead_code)]
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
