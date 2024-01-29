/*!
 * @lc app=leetcode.cn id=95 lang=rust
 *
 * # [95] 不同的二叉搜索树 II
 *
 * https://leetcode.cn/problems/unique-binary-search-trees-ii/description/
 *
 * algorithms
 * Medium (73.37%)
 * Likes:    1440
 * Dislikes: 0
 * Total Accepted:    170K
 * Total Submissions: 231.6K
 * Testcase Example:  '3'
 *
 * 给你一个整数 n ，请你生成并返回所有由 n 个节点组成且节点值从 1 到 n 互不相同的不同 二叉搜索树 。可以按 任意顺序 返回答案。
 *
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：n = 3
 * 输出：[[1,null,2,null,3],[1,null,3,2],[2,1,3],[3,1,null,null,2],[3,2,null,1]]
 *
 *
 * 示例 2：
 *
 *
 * 输入：n = 1
 * 输出：[[1]]
 *
 *
 *
 *
 * ## 提示：
 *
 *
 * 1
 *
 *
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
    /// ## 解题思路
    /// - 递归
    pub fn generate_trees2(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return Vec::new();
        }

        fn generate(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
            if start > end {
                return vec![None];
            }

            let mut res = Vec::new();
            for i in start..=end {
                let left_trees = generate(start, i - 1);
                let right_trees = generate(i + 1, end);
                for l in &left_trees {
                    for r in &right_trees {
                        let current_tree = Some(Rc::new(RefCell::new(TreeNode::new(i))));
                        current_tree.as_ref().unwrap().borrow_mut().left = l.clone();
                        current_tree.as_ref().unwrap().borrow_mut().right = r.clone();
                        res.push(current_tree)
                    }
                }
            }

            res
        }

        generate(1, n)
    }
}
// @lc code=end
