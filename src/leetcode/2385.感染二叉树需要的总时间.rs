/*!
 * @lc app=leetcode.cn id=2385 lang=rust
 *
 * # [2385] 感染二叉树需要的总时间
 *
 * https://leetcode.cn/problems/amount-of-time-for-binary-tree-to-be-infected/description/
 *
 * algorithms
 * Medium (46.26%)
 * Likes:    43
 * Dislikes: 0
 * Total Accepted:    8.6K
 * Total Submissions: 18.7K
 * Testcase Example:  '[1,5,3,null,4,10,6,9,2]\n3'
 *
 * 给你一棵二叉树的根节点 root ，二叉树中节点的值 互不相同 。另给你一个整数 start 。在第 0 分钟，感染 将会从值为 start
 * 的节点开始爆发。
 *
 * 每分钟，如果节点满足以下全部条件，就会被感染：
 *
 *
 * 节点此前还没有感染。
 * 节点与一个已感染节点相邻。
 *
 *
 * 返回感染整棵树需要的分钟数。
 *
 *
 *
 * ## 示例 1：
 *
 * 输入：root = [1,5,3,null,4,10,6,9,2], start = 3
 * 输出：4
 * 解释：节点按以下过程被感染：
 * - 第 0 分钟：节点 3
 * - 第 1 分钟：节点 1、10、6
 * - 第 2 分钟：节点5
 * - 第 3 分钟：节点 4
 * - 第 4 分钟：节点 9 和 2
 * 感染整棵树需要 4 分钟，所以返回 4 。
 *
 *
 * ## 示例 2：
 *
 * 输入：root = [1], start = 1
 * 输出：0
 * 解释：第 0 分钟，树中唯一一个节点处于感染状态，返回 0 。
 *
 *
 *
 *
 * ## 提示：
 *
 *
 * 树中节点的数目在范围 [1, 10^5] 内
 * 1 <= Node.val <= 10^5
 * 每个节点的值 互不相同
 * 树中必定存在值为 start 的节点
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
    /// - 树遍历
    /// 1. 如果起始节点为根节点, 则感染时间为树的高度(左右子树高度大的那个+1);
    /// 2. 否则, 感染时间为根节点到起始节点时间+
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        use std::collections::VecDeque;

        match &root {
            None => return 0,
            Some(node) => {
                if node.as_ref().borrow().val == start {
                    return get_height(root.clone(), start).0;
                } else {
                    let (hl, hls) = get_height(node.as_ref().borrow().left.clone(), start);
                    let (hr, hrs) = get_height(node.as_ref().borrow().right.clone(), start);
                    if hls > -2 {
                        return (hr + 1 + hls + 1).max(hl - hls);
                    } else {
                        return (hl + 1 + hrs + 1).max(hr - hrs);
                    }
                }
            }
        }

        fn get_height(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> (i32, i32) {
            let mut q = VecDeque::new();
            let mut h = -1;
            let mut h_start = -2;
            root.map(|root| {
                q.push_back(root);
                while !q.is_empty() {
                    h += 1;
                    for _ in 0..q.len() {
                        q.pop_front().map(|node| {
                            if node.borrow().val == start {
                                h_start = h;
                            }
                            node.borrow().left.clone().map(|n| q.push_back(n));
                            node.borrow().right.clone().map(|n| q.push_back(n));
                        });
                    }
                }
            });
            (h, h_start)
        }
    }
}
// @lc code=end
