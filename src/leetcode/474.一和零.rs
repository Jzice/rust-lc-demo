/*!
 * # [474.一和零](https://leetcode.cn/problems/ones-and-zeroes/description/)
 *
 * @lc app=leetcode.cn id=474 lang=rust
 *
 * ## 难度
 *
 * - Medium (65.13%)
 * - Likes:    982
 * - Dislikes: 0
 * - Total Accepted:    162.2K
 * - Total Submissions: 248.9K
 * - Testcase Example:  '["10","0001","111001","1","0"]\n5\n3'
 *
 * ## 问题描述
 *
 * 给你一个二进制字符串数组 strs 和两个整数 m 和 n 。
 *
 * 请你找出并返回 strs 的最大子集的长度，该子集中 最多 有 m 个 0 和 n 个 1 。
 *
 * 如果 x 的所有元素也是 y 的元素，集合 x 是集合 y 的 子集 。
 *
 *
 * ## 示例 1：
 *
 * - 输入：strs = ["10", "0001", "111001", "1", "0"], m = 5, n = 3
 * - 输出：4
 * - 解释：最多有 5 个 0 和 3 个 1 的最大子集是 {"10","0001","1","0"} ，因此答案是 4 。
 * - 其他满足题意但较小的子集包括 {"0001","1"} 和 {"10","1","0"} 。{"111001"} 不满足题意，因为它含 4 个 1
 * ，大于 n 的值 3 。
 *
 *
 * ## 示例 2：
 *
 * - 输入：strs = ["10", "0", "1"], m = 1, n = 1
 * - 输出：2
 * - 解释：最大的子集是 {"0", "1"} ，所以答案是 2 。
 *
 *
 * 提示：
 *
 * - 1 <= strs.length <= 600
 * - 1 <= strs[i].length <= 100
 * - strs[i] 仅由 '0' 和 '1' 组成
 * - 1 <= m, n <= 100
 *
 */

use super::*;

// @lc code=start
impl Solution {
    /// # 一和零
    /// ## 解题思路
    /// - 动态规划(0-1背包问题)
    /// 1. 令 dp[i][j]: 表示最多i个0,j个1的最大子集数;
    /// 2. 递推关系: dp[i][j] = max(dp[i-z][j-o]+1)  (i=[z..=m], j=[o..=n]);
    /// 3. 初始条件: dp[0][0] = 0;
    /// 4. 终止条件: dp[m][n];
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        fn str_01_count(str: &str) -> (usize, usize) {
            str.chars().fold((0, 0), |(mut z, mut o), c| {
                if c == '0' {
                    z += 1;
                } else {
                    o += 1;
                }
                (z, o)
            })
        }
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![0_i32; n + 1]; m + 1];
        for str in &strs {
            let (z, o) = str_01_count(str);
            for i in (z..=m).rev() {
                for j in (o..=n).rev() {
                    dp[i][j] = dp[i][j].max(dp[i - z][j - o] + 1);
                }
            }
        }
        dp[m][n]
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::find_max_form(vec!["10".into(), "0".into(), "1".into()], 1, 1), 2);
    }
}
