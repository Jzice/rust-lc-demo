/*!
 * # [129.求根节点到叶节点数字之和](https://leetcode.cn/problems/sum-root-to-leaf-numbers/description/)
 *
 * @lc app=leetcode.cn id=129 lang=rust
 *
 * ## 难度
 * - Medium (70.09%)
 * - Likes:    663
 * - Dislikes: 0
 * - Total Accepted:    220.1K
 * - Total Submissions: 314K
 * - Testcase Example:  '[1,2,3]'
 *
 * ## 描述
 *
 * 给你一个二叉树的根节点 root ，树中每个节点都存放有一个 0 到 9 之间的数字。
 *
 * 每条从根节点到叶节点的路径都代表一个数字：
 *
 * 例如，从根节点到叶节点的路径 1 -> 2 -> 3 表示数字 123 。
 *
 * 计算从根节点到叶节点生成的 所有数字之和 。
 *
 * 叶节点 是指没有子节点的节点。
 *
 *
 *
 * ## 示例 1：
 * - 输入：root = [1,2,3]
 * - 输出：25
 * - 解释：
 *   - 从根到叶子节点路径 1->2 代表数字 12
 *   - 从根到叶子节点路径 1->3 代表数字 13
 *   - 因此，数字总和 = 12 + 13 = 25
 *
 * ## 示例 2：
 * - 输入：root = [4,9,0,5,1]
 * - 输出：1026
 * - 解释：
 *   - 从根到叶子节点路径 4->9->5 代表数字 495
 *   - 从根到叶子节点路径 4->9->1 代表数字 491
 *   - 从根到叶子节点路径 4->0 代表数字 40
 *   - 因此，数字总和 = 495 + 491 + 40 = 1026
 *
 * ## 提示：
 * - 树中节点的数目在范围 [1, 1000] 内
 * - 树的深度不超过 10
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
    /// # 求根节点到叶节点数字之和
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        type TreeNodeOpt = Option<Rc<RefCell<TreeNode>>>;

        // 中序遍历, 递归计算各路径和
        fn inorder(root: &TreeNodeOpt, path_sum: i32, res: &mut i32) {
            match root {
                None => {}
                Some(node) => {
                    let n = node.borrow();
                    match (&n.left, &n.right) {
                        (None, None) => {
                            *res += path_sum * 10 + n.val;
                        }
                        _ => {
                            inorder(&n.left, path_sum * 10 + n.val, res);
                            inorder(&n.right, path_sum * 10 + n.val, res);
                        }
                    }
                }
            }
        }

        let mut res = 0;
        inorder(&root, 0, &mut res);

        res
    }
}
// @lc code=end
