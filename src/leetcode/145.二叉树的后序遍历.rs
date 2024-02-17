/*!
 * # [145.二叉树的后序遍历](https://leetcode.cn/problems/binary-tree-postorder-traversal/description/)
 *
 * @lc app=leetcode.cn id=145 lang=rust
 *
 * ## 难度
 * - Easy (75.09%)
 * - Likes:    709
 * - Dislikes: 0
 * - Total Accepted:    323.2K
 * - Total Submissions: 430K
 * - Testcase Example:  '[1,null,2,3]'
 *
 * ## 问题描述
 *
 * 给定一个二叉树，返回它的 后序 遍历。
 *
 * ## 示例:
 * - 输入: [1,null,2,3]
 * ```text
 *   1
 *    \
 *     2
 *    /
 *   3
 * ```
 * - 输出: [3,2,1]
 *
 * ## 进阶: 
 * 
 * 递归算法很简单，你可以通过迭代算法完成吗？
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
use std::iter;
use std::rc::Rc;
impl Solution {
    /// # 145.二叉树的后序遍历
    /// ## 解题思路
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        /// - 递归
        /// 1. 后序遍历的顺序： 左子树 -> 右子树 -> 根节点
        fn traversal(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            root.as_ref().map(|n| {
                traversal(&n.borrow().left, res);
                traversal(&n.borrow().right, res);
                res.push(n.borrow().val);
            });
        }

        /// - 迭代器
        fn postorder_iter(
            root: &Option<Rc<RefCell<TreeNode>>>,
        ) -> Box<dyn iter::Iterator<Item = i32>> {
            root.as_ref().map_or(Box::new(iter::empty()), |root| {
                Box::new(
                    postorder_iter(&root.borrow().left)
                        .chain(postorder_iter(&root.borrow().right))
                        .chain(iter::once(root.borrow().val)),
                )
            })
        }

        /// - 非递归/栈
        fn postorder_traversal_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut res = vec![];
            let mut stack = vec![];

            let mut visited = None;
            let mut node = root;
            loop {
                //先深度遍历节点的左子树，将遍历过的节点放入栈中
                while let Some(n) = node {
                    stack.push(n.clone());
                    node = n.borrow().left.clone();
                }
                //栈为空，则无待处理的节点，退出迭代
                if stack.is_empty() {
                    break;
                }
                // 检查栈顶元素
                //
                let r = stack.last().unwrap().borrow().right.clone();
                if r != visited {
                    node = r;
                    visited = None
                } else {
                    //右子树已被遍历过
                    if let Some(n) = stack.pop() {
                        res.push(n.borrow().val);
                        visited = Some(n.clone());
                    }
                }
            }
            res
        }

        // let mut res = vec![];
        // traversal(&root, &mut res);
        // res

        postorder_iter(&root).collect()
    }
}
// @lc code=end
