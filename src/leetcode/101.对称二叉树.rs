/*!
 * @lc app=leetcode.cn id=101 lang=rust
 *
 * # [101] 对称二叉树
 *
 * https://leetcode.cn/problems/symmetric-tree/description/
 *
 * algorithms
 * Easy (58.88%)
 * Likes:    2485
 * Dislikes: 0
 * Total Accepted:    865.4K
 * Total Submissions: 1.5M
 * Testcase Example:  '[1,2,2,3,4,4,3]'
 *
 * 给你一个二叉树的根节点 root ， 检查它是否轴对称。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：root = [1,2,2,3,4,4,3]
 * 输出：true
 *
 *
 * 示例 2：
 *
 *
 * 输入：root = [1,2,2,null,3,null,3]
 * 输出：false
 *
 *
 *
 *
 * 提示：
 *
 *
 * 树中节点数目在范围 [1, 1000] 内
 * -100 <= Node.val <= 100
 *
 *
 *
 *
 * 进阶：你可以运用递归和迭代两种方法解决这个问题吗？
 *
 */

use super::*;

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// ## 迭代
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root = &root;
        if let Some(node) = root {
            let mut q = vec![];
            q.push(node.borrow().left.clone());
            q.push(node.borrow().right.clone());
            while !q.is_empty() {
                match (q.pop(), q.pop()) {
                    (Some(a), Some(b)) => match (a, b) {
                        (Some(a), Some(b)) => {
                            if a.borrow().val != b.borrow().val {
                                return false;
                            } else {
                                q.push(a.borrow().left.clone());
                                q.push(b.borrow().right.clone());
                                q.push(a.borrow().right.clone());
                                q.push(b.borrow().left.clone());
                            }
                        }
                        (None, None) => {}
                        _ => return false,
                    },
                    (None, None) => return true,
                    _ => return false,
                }
            }

            return true;
        } else {
            return true;
        }
    }

    /// ## 递归
    pub fn is_symmetric2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        /// a, b是否为镜像树
        fn is_mirror(a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (a, b) {
                (None, None) => true,
                (Some(a), Some(b)) => {
                    a.borrow().val == b.borrow().val
                        && is_mirror(a.borrow().left.clone(), b.borrow().right.clone())
                        && is_mirror(a.borrow().right.clone(), b.borrow().left.clone())
                }
                _ => false,
            }
        }

        match root {
            None => true,
            Some(root) => is_mirror(root.borrow().left.clone(), root.borrow().right.clone()),
        }
    }
}
// @lc code=end
