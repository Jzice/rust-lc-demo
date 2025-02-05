/*!
 * # [1723.完成所有工作的最短时间] ( https://leetcode.cn/problems/find-minimum-time-to-finish-all-jobs/description/)
 *
 * @lc app=leetcode.cn id=1723 lang=rust
 *
 * ## 难度
 * - Hard (50.81%)
 * - Total Accepted:    29.2K
 * - Total Submissions: 57.4K
 * - Testcase Example:  '[3,2,3]\n3'
 *
 * ## 描述
 *
 * 给你一个整数数组 jobs ，其中 jobs[i] 是完成第 i 项工作要花费的时间。
 *
 * 请你将这些工作分配给 k 位工人。所有工作都应该分配给工人，且每项工作只能分配给一位工人。工人的 工作时间
 * 是完成分配给他们的所有工作花费时间的总和。请你设计一套最佳的工作分配方案，使工人的 最大工作时间 得以 最小化 。
 *
 * 返回分配方案中尽可能 最小 的 最大工作时间 。
 *
 *
 * ## 示例 1：
 * - 输入：jobs = [3,2,3], k = 3
 * - 输出：3
 * - 解释：给每位工人分配一项工作，最大工作时间是 3 。
 *
 *
 * ## 示例 2：
 * - 输入：jobs = [1,2,4,7,8], k = 2
 * - 输出：11
 * - 解释：按下述方式分配工作：
 *   - 1 号工人：1、2、8（工作时间 = 1 + 2 + 8 = 11）
 *   - 2 号工人：4、7（工作时间 = 4 + 7 = 11）
 *   - 最大工作时间是 11 。
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 二分查找
    /// - 二分查找的上下界
    /// - 二分查找的判断条件
    /// - 二分查找的更新方式
    /// - 二分查找的返回值
    /// - 二分查找的边界条件
    /// - 二分查找的优化
    /// - 二分查找的注意事项
    pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
        todo!();
    }
}

// @lc code=end
