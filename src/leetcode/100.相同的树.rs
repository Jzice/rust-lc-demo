/*!
 * # [100.相同的树](https://leetcode.cn/problems/same-tree/description/)
 *
 * @lc app=leetcode.cn id=100 lang=rust
 *
 * ## 难度
 * - Easy (60.01%)
 * - Likes:    1045
 * - Dislikes: 0
 * - Total Accepted:    473.7K
 * - Total Submissions: 789.2K
 * - Testcase Example:  '[1,2,3]\n[1,2,3]'
 *
 * ## 问题描述
 *
 * 给你两棵二叉树的根节点 p 和 q ，编写一个函数来检验这两棵树是否相同。
 *
 * 如果两个树在结构上相同，并且节点具有相同的值，则认为它们是相同的。
 *
 *
 * ## 示例 1：
 *
 * - 输入：p = [1,2,3], q = [1,2,3]
 * - 输出：true
 *
 *
 * ## 示例 2：
 *
 * - 输入：p = [1,2], q = [1,null,2]
 * - 输出：false
 *
 *
 * ## 示例 3：
 * 
 * - 输入：p = [1,2,1], q = [1,1,2]
 * - 输出：false
 *
 * ## 提示：
 *
 * - 两棵树上的节点数目都在范围 [0, 100] 内
 * -10^4
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
    /// # 相同的树
    /// ## 基本思想
    /// - 递归
    /// 1. 二叉树相等, 包含以下条件：
    /// - 根节点值相等;
    /// - 左右子树都相等;
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                p.borrow().val == q.borrow().val
                    && Solution::is_same_tree(p.borrow().left.clone(), q.borrow().left.clone())
                    && Solution::is_same_tree(p.borrow().right.clone(), q.borrow().right.clone())
            }
            _ => false,
        }
    }
}
// @lc code=end
//
#[cfg(test)]
mod tests {
    use super::*;
    use crate::btree;
    use crate::ds::binary_tree::to_tree;

    #[test]
    fn test() {
        assert_eq!( 
            Solution::is_same_tree( 
                btree![Some(1), Some(2), Some(3)], 
                btree![Some(1), Some(2), Some(3)] 
            ), 
            true
        );
        assert_eq!( 
            Solution::is_same_tree( 
                btree![Some(1), Some(2), Some(3)], 
                btree![Some(1), Some(2), Some(4)] 
            ), 
            false
        );
        assert_eq!( 
            Solution::is_same_tree( 
                btree![Some(1), Some(2), Some(3)], 
                btree![Some(1), None, Some(3)] 
            ), 
            false
        );
        assert_eq!( 
            Solution::is_same_tree(
                btree![Some(1), Some(2)], 
                btree![Some(1), None, Some(2)], 
            ),
            false
        );
        assert_eq!( 
            Solution::is_same_tree(
                btree![Some(1), Some(2)], 
                btree![Some(1), Some(2), None], 
            ),
            true
        );
    }
}
