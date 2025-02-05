/*!
 * # [108.将有序数组转换为二叉搜索树](https://leetcode.cn/problems/convert-sorted-array-to-binary-search-tree/description/)
 *
 * @lc app=leetcode.cn id=108 lang=rust
 *
 * ## 难度
 * - Easy (77.50%)
 * - Likes:    1359
 * - Dislikes: 0
 * - Total Accepted:    375.7K
 * - Total Submissions: 484.6K
 * - Testcase Example:  '[-10,-3,0,5,9]'
 *
 * ## 问题描述
 *
 * 给你一个整数数组 nums ，其中元素已经按 升序 排列，请你将其转换为一棵 高度平衡 二叉搜索树。
 *
 * 高度平衡 二叉树是一棵满足「每个节点的左右两个子树的高度差的绝对值不超过 1 」的二叉树。
 *
 *
 * ## 示例 1：
 * - 输入：nums = [-10,-3,0,5,9]
 * - 输出：[0,-3,9,-10,null,5]
 * - 解释：[0,-10,5,null,-3,null,9] 也将被视为正确答案：
 *
 * ## 示例 2：
 * - 输入：nums = [1,3]
 * - 输出：[3,1]
 * - 解释：[1,null,3] 和 [3,1] 都是高度平衡二叉搜索树。
 *
 * ## 提示：
 * - 1 <= nums.length <= 10^4
 * - -10^4 <= nums[i] <= 10^4
 * - nums 按 严格递增 顺序排列
 *
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
    /// # 将有序数组转换为二叉搜索树
    /// - 递归
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        /// 
        fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.len() == 0 {
                return None;
            }
            let i = nums.len() / 2;
            Some(Rc::new(RefCell::new(TreeNode {
                val: nums[i],
                left: helper(&nums[..i]),
                right: helper(&nums[(i + 1)..]),
            })))
        }
        helper(&nums)
    }
}
// @lc code=end
