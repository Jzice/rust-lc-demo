/*!
 * # [889.根据前序和后序遍历构造二叉树](https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-postorder-traversal/description/)
 *
 * @lc app=leetcode.cn id=889 lang=rust
 *
 * ## 难度
 * Medium (68.09%)
 * Likes:    299
 * Dislikes: 0
 * Total Accepted:    36K
 * Total Submissions: 52.9K
 * Testcase Example:  '[1,2,4,5,3,6,7]\n[4,5,2,6,7,3,1]'
 *
 * ## 描述
 *
 * 给定两个整数数组，preorder 和 postorder ，其中 preorder 是一个具有 无重复 值的二叉树的前序遍历，postorder是同一棵树的后序遍历，重构并返回二叉树。
 *
 * 如果存在多个答案，您可以返回其中 任何 一个。
 *
 *
 * ## 示例 1：
 * - 输入：preorder = [1,2,4,5,3,6,7], postorder = [4,5,2,6,7,3,1]
 * - 输出：[1,2,3,4,5,6,7]
 *
 *
 * ## 示例 2:
 * - 输入: preorder = [1], postorder = [1]
 * - 输出: [1]
 *
 *
 * ## 提示：
 * - 1 <= preorder.length <= 30
 * - 1 <= preorder[i] <= preorder.length
 * - preorder 中所有值都 不同
 * - postorder.length == preorder.length
 * - 1 <= postorder[i] <= postorder.length
 * - postorder 中所有值都 不同
 * - 保证 preorder 和 postorder 是同一棵二叉树的前序遍历和后序遍历
 *
 *
 */

use super::*;
struct Solution;

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
    /// ## 解题思路
    /// - 递归
    /// 1. 前序遍历： 根->左子树->右子树
    ///    后序遍历:  左子树->右子树->根
    /// 2. 所以数组的首尾为树的根，
    ///    preorder的[1..]和postorder的[..len-1]分别为两种左子树+右子树；
    /// 3. 每个子树的preorder slice第一个元素为左子树root，其必定在postorder的左子树最后；
    /// 4. 据此查找该子树根在postorder中的index，来将左右子树进划分；
    /// 5. 递归求出左右子树；
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        assert!(preorder.len() > 0);

        let root_val = preorder[0];
        let mut root = TreeNode::new(root_val);
        let len = preorder.len();
        // 元素个数小于2个
        if preorder.len() <= 1 {
            return Some(Rc::new(RefCell::new(root)));
        }

        let cut_idx = postorder.iter().position(|&x| x == preorder[1]).unwrap() + 1;
        let pre_left = &preorder[1..(cut_idx + 1)];
        let post_left = &postorder[0..cut_idx];
        root.left = Solution::construct_from_pre_post(pre_left.to_vec(), post_left.to_vec());

        if cut_idx < preorder.len() - 1 {
            let pre_right = &preorder[(cut_idx + 1)..len];
            let post_right = &postorder[cut_idx..len - 1];
            root.right = Solution::construct_from_pre_post(pre_right.to_vec(), post_right.to_vec());
        }

        Some(Rc::new(RefCell::new(root)))
    }
}
// @lc code=end
