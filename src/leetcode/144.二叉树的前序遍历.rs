/*!
 * @lc app=leetcode.cn id=144 lang=rust
 *
 * # [144] 二叉树的前序遍历
 *
 * https://leetcode.cn/problems/binary-tree-preorder-traversal/description/
 *
 * algorithms
 * Easy (70.42%)
 * Likes:    666
 * Dislikes: 0
 * Total Accepted:    428.8K
 * Total Submissions: 608.9K
 * Testcase Example:  '[1,null,2,3]'
 *
 * 给你二叉树的根节点 root ，返回它节点值的 前序 遍历。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：root = [1,null,2,3]
 * 输出：[1,2,3]
 *
 *
 * 示例 2：
 *
 *
 * 输入：root = []
 * 输出：[]
 *
 *
 * 示例 3：
 *
 *
 * 输入：root = [1]
 * 输出：[1]
 *
 *
 * 示例 4：
 *
 *
 * 输入：root = [1,2]
 * 输出：[1,2]
 *
 *
 * 示例 5：
 *
 *
 * 输入：root = [1,null,2]
 * 输出：[1,2]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 树中节点数目在范围 [0, 100] 内
 * -100
 *
 *
 *
 *
 * 进阶：递归算法很简单，你可以通过迭代算法完成吗？
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
    /// - 前序遍历顺序： 根节点 -> 左子树 -> 右子树
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        /// - 递归
        fn preorder_rec(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            root.as_ref().map(|n| {
                res.push(n.borrow().val);
                preorder_rec(&n.borrow().left, res);
                preorder_rec(&n.borrow().right, res);
            });
        }

        /// - 迭代器
        fn preorder_iter(
            root: &Option<Rc<RefCell<TreeNode>>>,
        ) -> Box<dyn iter::Iterator<Item = i32>> {
            root.as_ref().map_or(Box::new(iter::empty()), |root| {
                Box::new(
                    iter::once(root.borrow().val)
                        .chain(preorder_iter(&root.borrow().left))
                        .chain(preorder_iter(&root.borrow().right)),
                )
            })
        }

        /// - 非递归/栈
        fn preorder_traversal_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut res = vec![];
            let mut stack = vec![];
            let mut node = root;
            loop {
                //先遍历每一个节点的左子树，并将节点放入栈中
                while let Some(n) = node {
                    res.push(n.borrow().val); //现将当前节点输出

                    stack.push(n.clone()); //
                    node = n.borrow().left.clone();
                }

                //栈为空，则无待处理的节点，退出迭代
                if stack.is_empty() {
                    break;
                }
                //左子树处理完

                //处理栈中节点
                stack.pop().map(|n| {
                    node = n.borrow().right.clone();
                });
            }

            res
        }

        // let mut res = Vec::new();
        // preorder_rec(&root, &mut res);
        // res

        preorder_iter(&root).collect()
    }
}
// @lc code=end
