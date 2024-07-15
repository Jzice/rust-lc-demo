/*!
 * # [120.三角形最小路径和](https://leetcode.cn/problems/triangle/description/)
 *
 * @lc app=leetcode.cn id=120 lang=rust
 *
 * ## 难度
 * - Medium (68.65%)
 * - Likes:    1243
 * - Dislikes: 0
 * - Total Accepted:    301.4K
 * - Total Submissions: 439.1K
 * - Testcase Example:  '[[2],[3,4],[6,5,7],[4,1,8,3]]'
 *
 * ## 问题描述
 *
 * 给定一个三角形 triangle ，找出自顶向下的最小路径和。
 *
 * 每一步只能移动到下一行中相邻的结点上。相邻的结点 在这里指的是 下标 与 上一层结点下标 相同或者等于 上一层结点下标 + 1
 * 的两个结点。也就是说，如果正位于当前行的下标 i ，那么下一步可以移动到下一行的下标 i 或 i + 1 。
 *
 * ## 示例 1：
 * - 输入：triangle = [[2],[3,4],[6,5,7],[4,1,8,3]]
 * - 输出：11
 * - 解释：如下面简图所示：
 * ```text
 *    2
 *   3 4
 *  6 5 7
 * 4 1 8 3
 * ```
 * 自顶向下的最小路径和为 11（即，2 + 3 + 5 + 1 = 11）。
 *
 *
 * ## 示例 2：
 * - 输入：triangle = [[-10]]
 * - 输出：-10
 *
 * ## 提示：
 * - triangle[0].length == 1
 * - triangle[i].length == triangle[i - 1].length + 1
 * - -10^4
 *
 * ## 进阶：
 * 你可以只使用 O(n) 的额外空间（n 为三角形的总行数）来解决这个问题吗？
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// # 三角形最小路径和
    /// ## 解题思路
    /// - 动态规划
    /// 1. 设 dp[i][j]: 从triangle[i][j]出发到最底边的最小路径;
    /// 2. 目标: dp[0][0]
    /// 3. 递推关系:   
    ///      dp[i][j] = min(dp[i+1][j], dp[i+1][j+1]) + triangle[i][j]
    /// 4. 初始条件:
    ///      dp[n][i] = triangle[n][i];
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        if n == 1 {
            return triangle[0][0];
        }

        let mut dp = vec![vec![0; n + 1]; n + 1];
        for i in 0..n {
            dp[n - 1][i] = triangle[n - 1][i];
        }
        for r in (0..n).rev() {
            for c in 0..=r {
                dp[r][c] = triangle[r][c] + dp[r + 1][c].min(dp[r + 1][c + 1]);
            }
        }

        dp[0][0]
    }
}
// @lc code=end
