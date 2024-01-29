/*!
* @lc app=leetcode.cn id=1123 lang=rust slug=lowest-common-ancestor-of-deepest-leaves
*
* # 1123.最深叶节点的最近公共祖先
*
* https://leetcode.cn/problems/lowest-common-ancestor-of-deepest-leaves/description/
*
* Medium (74.28%)
*
给你一个有根节点 root 的二叉树，返回它 最深的叶节点的最近公共祖先 。
回想一下：
    叶节点 是二叉树中没有子节点的节点
    树的根节点的 深度 为 0，如果某一节点的深度为 d，那它的子节点的深度就是 d+1
    如果我们假定 A 是一组节点 S 的 最近公共祖先，S 中的每个节点都在以 A 为根节点的子树中，且 A 的深度达到此条件下可能的最大值。

示例 1：
https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/01/sketch1.png
输入：root = [3,5,1,6,2,0,8,null,null,7,4]
输出：[2,7,4]
解释：我们返回值为 2 的节点，在图中用黄色标记。
在图中用蓝色标记的是树的最深的节点。
注意，节点 6、0 和 8 也是叶节点，但是它们的深度是 2 ，而节点 7 和 4 的深度是 3 。
示例 2：
输入：root = [1]
输出：[1]
解释：根节点是树中最深的节点，它是它本身的最近公共祖先。
示例 3：
输入：root = [0,1,3,null,2]
输出：[2]
解释：树中最深的叶节点是 2 ，最近公共祖先是它自己。

提示：
    树中的节点数将在 [1, 1000] 的范围内。
    0 <= Node.val <= 1000
    每个节点的值都是 独一无二 的。

注意：本题与力扣 865 重复：https://leetcode-cn.com/problems/smallest-subtree-with-all-the-deepest-nodes/
*
* test case:
[3,5,1,6,2,0,8,null,null,7,4]
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
    /// 1. 如果root为None, 则结果为None;
    /// 2. 否则root不为空, left, right分别为其左右子树根节点;
    /// 3. 如果left, right均为None, 则当前节点为叶子节点, 返回该节点;
    /// 4. 如果left为None, right不为None, 则最深叶节点的最近公共祖先一定在right中,
    ///    递归求取以right为根的lca;
    /// 5. 如果left不为None, right为None, 则递归left;
    /// 6. 否则left, right都不为None, 则须判断left,right的高度
    /// 6.1. 如果left_heigh == right_heigh, 则当前节点为lca;
    /// 6.2. 否则递归heigh大的那一个;
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // 获取树的高度
        fn get_heigh(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match root {
                None => 0,
                Some(root) => {
                    get_heigh(&root.borrow().left).max(get_heigh(&root.borrow().right)) + 1
                }
            }
        }
        match root {
            None => None,
            Some(root) => match (root.borrow().left.clone(), root.borrow().right.clone()) {
                (None, None) => Some(root.clone()),
                (Some(left), None) => Solution::lca_deepest_leaves(Some(left.clone())),
                (None, Some(right)) => Solution::lca_deepest_leaves(Some(right.clone())),
                (Some(left), Some(right)) => {
                    let left_heigh = get_heigh(&Some(left.clone()));
                    let right_heigh = get_heigh(&Some(right.clone()));
                    if left_heigh == right_heigh {
                        return Some(root.clone());
                    } else if left_heigh > right_heigh {
                        return Solution::lca_deepest_leaves(Some(left.clone()));
                    } else {
                        return Solution::lca_deepest_leaves(Some(right.clone()));
                    }
                }
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
