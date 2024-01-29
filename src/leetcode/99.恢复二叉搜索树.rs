/*!
 * @lc app=leetcode.cn id=99 lang=rust
 *
 * # [99] 恢复二叉搜索树
 *
 * https://leetcode.cn/problems/recover-binary-search-tree/description/
 *
 * algorithms
 * Medium (60.26%)
 * Likes:    872
 * Dislikes: 0
 * Total Accepted:    138.9K
 * Total Submissions: 230.4K
 * Testcase Example:  '[1,3,null,null,2]'
 *
 * 给你二叉搜索树的根节点 root ，该树中的 恰好 两个节点的值被错误地交换。请在不改变其结构的情况下，恢复这棵树 。
 *
 *
 *
 * ## 示例 1：
 *
 *
 * 输入：root = [1,3,null,null,2]
 * 输出：[3,1,null,null,2]
 * 解释：3 不能是 1 的左孩子，因为 3 > 1 。交换 1 和 3 使二叉搜索树有效。
 *
 *
 * ## 示例 2：
 *
 *
 * 输入：root = [3,1,4,null,null,2]
 * 输出：[2,1,4,null,null,3]
 * 解释：2 不能在 3 的右子树中，因为 2 < 3 。交换 2 和 3 使二叉搜索树有效。
 *
 *
 *
 * 提示：
 *
 *
 * 树上节点的数目在范围 [2, 1000] 内
 * -2^31 <= Node.val <= 2^31 - 1
 *
 *
 *
 *
 * ## 进阶：
 *
 * 使用 O(n) 空间复杂度的解法很容易实现。你能想出一个只使用 O(1) 空间的解决方案吗？
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
    /// 1. 二叉搜索树进行中序遍历时, 各节点值将保持顺序递增, 即node[i].val < node[i+1].val;
    /// 2. 如果有两个节点进行了交换, 则交换的两个节点值与周围节点将不满足顺序递增的关系;
    ///    假设交换前的两个节点node1, node2, 有 node1.val< node1.1.val < .. < node2.0.val < node2.val,
    ///    则交换后, [(node2.val) > node1.1.val] < .. < [node2.0.val > (node1.val)]
    /// 3. 所以, 中序遍历树时, 检查当前节点和前一个节点的大小关系,
    ///    当出现了第一次逆序, 则记录前一个节点node1,
    ///    出现第二次逆序时, 记录后一个节点node2,
    ///    之后交换node1, node2的值
    /// 4. 当被交换的两个节点node1, node2挨着时, 之后出现一次逆序情况,
    ///    此时直接记录node1为前一个节点, node2为后一个节点, 综合3,4两种情况,
    ///    则当发生逆序时, 逆序前一个节点只须更新一次, 后一个节点必须每次都更新;
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        /// 查找逆序节点对
        fn find_unorder_pairs(
            root: &Option<Rc<RefCell<TreeNode>>>,
            prev: &mut Option<Rc<RefCell<TreeNode>>>,
            unorder_pairs: &mut (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>),
        ) {
            if let Some(node) = root {
                let node = node.borrow();

                // 递归处理当前节点左子树
                find_unorder_pairs(&node.left, prev, unorder_pairs);

                // 处理当前节点
                match prev {
                    // 出现了逆序节点
                    Some(prev1) if prev1.borrow().val > node.val => {
                        if unorder_pairs.0.is_none() {
                            //node1只更新一次记录
                            unorder_pairs.0 = prev.clone();
                        }
                        // node2每次都得更新记录
                        unorder_pairs.1 = root.clone();
                    }
                    _ => {}
                }
                *prev = root.clone();

                // 递归处理当前节点左子树
                find_unorder_pairs(&node.right, prev, unorder_pairs);
            }
        }

        let mut pairs = (None, None);
        find_unorder_pairs(root, &mut None, &mut pairs);
        if let (Some(mut node1), Some(mut node2)) = pairs {
            std::mem::swap(&mut node1.borrow_mut().val, &mut node2.borrow_mut().val);
        }
    }
}
// @lc code=end
