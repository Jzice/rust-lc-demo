/*!
 * # [538.把二叉搜索树转换为累加树](https://leetcode.cn/problems/convert-bst-to-greater-tree/description/)
 *
 * @lc app=leetcode.cn id=538 lang=rust
 *
 * ## 难度
 * - Medium (76.71%)
 * - Likes:    934
 * - Dislikes: 0
 * - Total Accepted:    245K
 * - Total Submissions: 319.4K
 * - Testcase Example:  '[4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]'
 *
 * ## 问题描述
 *
 * 给出二叉 搜索 树的根节点，该树的节点值各不相同，请你将其转换为累加树（Greater Sum Tree），使每个节点 node
 * 的新值等于原树中大于或等于 node.val 的值之和。
 *
 * 提醒一下，二叉搜索树满足下列约束条件：
 * - 节点的左子树仅包含键 小于 节点键的节点。
 * - 节点的右子树仅包含键 大于 节点键的节点。
 * - 左右子树也必须是二叉搜索树。
 *
 * 注意：本题和[1038](https://leetcode.cn/problems/binary-search-tree-to-greater-sum-tree/)相同
 *
 * ## 示例 1：
 * - 输入：[4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]
 * - 输出：[30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]
 *
 * ## 示例 2：
 * - 输入：root = [0,null,1]
 * - 输出：[1,null,1]
 *
 * ## 示例 3：
 * - 输入：root = [1,0,2]
 * - 输出：[3,3,2]
 *
 * ## 示例 4：
 * - 输入：root = [3,2,4,1]
 * - 输出：[7,9,4,10]
 *
 * ## 提示：
 * - 树中的节点数介于 0 和 10^4^ 之间。
 * - 每个节点的值介于 -10^4 和 10^4 之间。
 * - 树中的所有值 互不相同 。
 * - 给定的树为二叉搜索树。
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
    /// # 把二叉搜索树转换为累加树
    /// ## 解题思路
    /// - stack
    /// 1. 遍历二叉搜索树, 计算所有节点val的累积和sum;
    /// 2. 再一次中序二叉搜索树,
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        type TreeNodeOpt = Option<Rc<RefCell<TreeNode>>>;
        let mut tmp_nodes = Vec::new();
        let mut sum = 0; //节点值累积和
        let mut node = root.clone();
        // 中序遍历二叉树
        loop {
            // 将所有的左子节点入栈
            while let Some(n) = node {
                node = n.borrow_mut().left.clone();
                tmp_nodes.push(n);
            }
            // 弹出栈顶节点
            if let Some(n) = tmp_nodes.pop() {
                // 累加节点值
                sum += n.borrow().val;
                node = n.borrow_mut().right.clone();
            } else {
                // 没有待遍历节点
                break;
            }
        }

        let mut node = root.clone();
        // 中序遍历二叉树
        loop {
            // 将所有的左子节点入栈
            while let Some(n) = node {
                node = n.borrow().left.clone();
                tmp_nodes.push(n);
            }
            // 弹出栈顶节点
            if let Some(n) = tmp_nodes.pop() {
                // 处理当前节点
                let val = n.borrow().val;
                n.borrow_mut().val = sum;
                sum -= val;

                node = n.borrow().right.clone();
            } else {
                // 没有待遍历节点
                break;
            }
        }

        root
    }
}
// @lc code=end
