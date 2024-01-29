/*!
 * @lc app=leetcode.cn id=94 lang=rust
 *
 * # [94] 二叉树的中序遍历
 *
 * https://leetcode.cn/problems/binary-tree-inorder-traversal/description/
 *
 * algorithms
 * Medium (70.23%)
 * Likes:    406
 * Dislikes: 0
 * Total Accepted:    109.4K
 * Total Submissions: 155.6K
 * Testcase Example:  '[1,null,2,3]'
 *
 * 给定一个二叉树，返回它的中序 遍历。
 *
 * 示例:
 *
 * 输入: [1,null,2,3]
 * ⁠  1
 * ⁠   \
 * ⁠    2
 * ⁠   /
 * ⁠  3
 *
 * 输出: [1,3,2]
 *
 * 进阶: 递归算法很简单，你可以通过迭代算法完成吗？
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
    /// ## 解题思路
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        /// - 递归
        /// 中序遍历顺序: 左->根->右
        fn inorder_rec(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            node.as_ref().map(|n| {
                inorder_rec(&n.borrow().left, res);
                res.push(n.borrow().val);
                inorder_rec(&n.borrow().right, res);
            });
        }

        /// - 迭代器
        fn inorder_iter(
            node: &Option<Rc<RefCell<TreeNode>>>,
        ) -> Box<dyn iter::Iterator<Item = i32>> {
            node.as_ref().map_or(Box::new(iter::empty()), |rc| {
                let tree = rc.as_ref().borrow();
                Box::new(
                    inorder_iter(&tree.left)
                        .chain(iter::once(tree.val))
                        .chain(inorder_iter(&tree.right)),
                )
            })
        }

        /// - 非递归/栈
        fn inorder_stack(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut res = vec![];
            let mut stack = vec![];
            let mut node = root.clone();
            while node.is_some() || !stack.is_empty() {
                // 将所有左子节点入栈
                while let Some(n) = node {
                    node = n.borrow().left.clone();
                    stack.push(n);
                }
                // 弹出栈顶节点
                stack.pop().map(|n| {
                    // 处理当前节点
                    res.push(n.borrow().val);
                    // 处理右子树
                    node = n.borrow().right.clone();
                });
            }

            res
        }

        // let mut res = vec![];
        // inorder_rec(&root, &mut res);
        // res

        // inorder_iter(&root).collect()

        inorder_stack(&root)
    }
}
// @lc code=end
