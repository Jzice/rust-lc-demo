/*!
 * # [173.二叉搜索树迭代器](https://leetcode.cn/problems/binary-search-tree-iterator/description/)
 *
 * @lc app=leetcode.cn id=173 lang=rust
 *
 * ## 难度
 * - Medium (80.47%)
 * - Likes:    520
 * - Dislikes: 0
 * - Total Accepted:    78K
 * - Total Submissions: 96.9K
 * - Testcase Example:  '["BSTIterator","next","next","hasNext","next","hasNext","next","hasNext","next","hasNext"]\n' +
 * '[[[7,3,15,null,null,9,20]],[],[],[],[],[],[],[],[],[]]'
 *
 * ## 问题描述
 *
 * 实现一个二叉搜索树迭代器类BSTIterator ，表示一个按中序遍历二叉搜索树（BST）的迭代器：
 *
 * ```cpp
 * BSTIterator(TreeNode root); // 初始化 BSTIterator 类的一个对象。BST 的根节点 root
 *   // 会作为构造函数的一部分给出。指针应初始化为一个不存在于 BST 中的数字，且该数字小于 BST 中的任何元素。
 * boolean hasNext(); //如果向指针右侧遍历存在数字，则返回 true ；否则返回 false 。
 * int next(); //将指针向右移动，然后返回指针处的数字。
 * ```
 *
 *
 * 注意，指针初始化为一个不存在于 BST 中的数字，所以对 next() 的首次调用将返回 BST 中的最小元素。
 *
 * 你可以假设 next() 调用总是有效的，也就是说，当调用 next() 时，BST 的中序遍历中至少存在一个下一个数字。
 *
 * ## 示例：
 * - 输入
 * ["BSTIterator", "next", "next", "hasNext", "next", "hasNext", "next",
 * "hasNext", "next", "hasNext"]
 * [[[7, 3, 15, null, null, 9, 20]], [], [], [], [], [], [], [], [], []]
 * - 输出
 * [null, 3, 7, true, 9, true, 15, true, 20, false]
 *
 * - 解释
 * ```cpp
 * BSTIterator bSTIterator = new BSTIterator([7, 3, 15, null, null, 9, 20]);
 * bSTIterator.next();    // 返回 3
 * bSTIterator.next();    // 返回 7
 * bSTIterator.hasNext(); // 返回 True
 * bSTIterator.next();    // 返回 9
 * bSTIterator.hasNext(); // 返回 True
 * bSTIterator.next();    // 返回 15
 * bSTIterator.hasNext(); // 返回 True
 * bSTIterator.next();    // 返回 20
 * bSTIterator.hasNext(); // 返回 False
 * ```
 *
 * ## 提示：
 * - 树中节点的数目在范围 [1, 10^5] 内
 * - 最多调用 10^5 次 hasNext 和 next 操作
 *
 * ## 进阶：
 * 你可以设计一个满足下述条件的解决方案吗？next() 和 hasNext() 操作均摊时间复杂度为 O(1) ，并使用 O(h) 内存。其中 h
 * 是树的高度。
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
//

use std::cell::RefCell;
use std::rc::Rc;

struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>, //保存待遍历的节点
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    /// ## 解题思路
    /// - 栈
    /// - 二叉树中序遍历
    /// 1. 初始化时，先将根节点入栈；
    /// 2. 再循环将根节点的左子节点入栈；
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut ret_iter = BSTIterator { stack: vec![] };

        // 如果根节点不为空
        if let Some(root) = root.as_ref() {
            // 将根节点入栈
            ret_iter.stack.push(root.clone());

            let mut node = Some(root.clone());
            // 如果左子节点不为空
            while let Some(left) = node.unwrap().borrow_mut().left.as_ref() {
                // 将左子节点入栈
                ret_iter.stack.push(left.clone());
                // 移动到左子节点继续处理
                node = Some(left.clone());
            }
        }

        ret_iter
    }

    /// next
    /// 1. 判断stack中是否存在未遍历的节点；
    /// 2. 如果存在，则将该节点出栈；
    /// 3. 将节点值保存为返回值；
    /// 4. 如果该节点存在右子节点，则将右子节点入栈；
    /// 5. 如果右子节点存在左子节点，依次将所有的左子节点入栈；
    fn next(&mut self) -> i32 {
        let mut ret_val = -1;
        // 如果stack中存在节点
        if let Some(node) = self.stack.pop() {
            ret_val = node.borrow().val;
            //如果当前节点存在右子节点
            if let Some(right) = node.borrow_mut().right.as_ref() {
                // 将右子节点入栈
                self.stack.push(right.clone());

                let mut n = Some(right.clone());
                //如果右子节点存在左子节点
                while let Some(left) = n.unwrap().borrow_mut().left.as_ref() {
                    // 将左子节点入栈
                    self.stack.push(left.clone());
                    n = Some(left.clone());
                }
            }
        }

        ret_val
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

// @lc code=end

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test() {}
}
