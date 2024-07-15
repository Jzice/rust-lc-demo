/*!
 * # [606.根据二叉树创建字符串](https://leetcode.cn/problems/construct-string-from-binary-tree/description/)
 *
 * @lc app=leetcode.cn id=606 lang=rust slug=construct-string-from-binary-tree
 *
 * ## 难度
 *
 * Easy (62.14%)
 *
 * ## 问题描述
 *
 * 给你二叉树的根节点 root ，请你采用前序遍历的方式，将二叉树转化为一个由括号和整数组成的字符串，返回构造出的字符串。
 * 空节点使用一对空括号对 "()" 表示，转化后需要省略所有不影响字符串与原始二叉树之间的一对一映射关系的空括号对。
 * 
 * ## 示例 1：
 *
 * [](https://assets.leetcode.com/uploads/2021/05/03/cons1-tree.jpg)
 *
 * - 输入：root = [1,2,3,4]
 * - 输出："1(2(4))(3)"
 * - 解释：初步转化后得到 "1(2(4)())(3()())" ，但省略所有不必要的空括号对后，字符串应该是"1(2(4))(3)" 。
 *
 * ## 示例 2：
 *
 * [](https://assets.leetcode.com/uploads/2021/05/03/cons2-tree.jpg)
 *
 * - 输入：root = [1,2,3,null,4]
 * - 输出："1(2()(4))(3)"
 * - 解释：和第一个示例类似，但是无法省略第一个空括号对，否则会破坏输入与输出一一映射的关系。
 * 
 * ## 提示：
 * - 树中节点的数目范围是 [1, 104]
 * - -1000 <= Node.val <= 1000
 *
 * ## 测试用例
 * ```text
 * [1,2,3,4]
 * ```
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
    /// ## 根据二叉树创建字符串
    /// - 递归
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        match root {
            None => "()".into(),
            Some(root) => match (root.borrow().left.clone(), root.borrow().right.clone()) {
                (None, None) => format!("{}", root.borrow().val),
                (Some(left), None) => {
                    format!("{}({})", root.borrow().val, Solution::tree2str(Some(left)))
                }
                (None, Some(right)) => format!(
                    "{}()({})",
                    root.borrow().val,
                    Solution::tree2str(Some(right))
                ),
                (Some(left), Some(right)) => format!(
                    "{}({})({})",
                    root.borrow().val,
                    Solution::tree2str(Some(left)),
                    Solution::tree2str(Some(right)),
                ),
            },
        }
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {}
}
