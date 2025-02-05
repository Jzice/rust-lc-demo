/*
 * @lc app=leetcode.cn id=1449 lang=rust
 *
 * [1449] 数位成本和为目标值的最大数字
 *
 * https://leetcode.cn/problems/form-largest-integer-with-digits-that-add-up-to-target/description/
 *
 * algorithms
 * Hard (62.33%)
 * Likes:    164
 * Dislikes: 0
 * Total Accepted:    19.2K
 * Total Submissions: 30.8K
 * Testcase Example:  '[4,3,2,5,6,7,2,5,5]\n9'
 *
 * 给你一个整数数组 cost 和一个整数 target 。请你返回满足如下规则可以得到的 最大 整数：
 *
 *
 * 给当前结果添加一个数位（i + 1）的成本为 cost[i] （cost 数组下标从 0 开始）。
 * 总成本必须恰好等于 target 。
 * 添加的数位中没有数字 0 。
 *
 *
 * 由于答案可能会很大，请你以字符串形式返回。
 *
 * 如果按照上述要求无法得到任何整数，请你返回 "0" 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：cost = [4,3,2,5,6,7,2,5,5], target = 9
 * 输出："7772"
 * 解释：添加数位 '7' 的成本为 2 ，添加数位 '2' 的成本为 3 。所以 "7772" 的代价为 2*3+ 3*1 = 9 。 "977"
 * 也是满足要求的数字，但 "7772" 是较大的数字。
 * ⁠数字     成本
 * ⁠ 1  ->   4
 * ⁠ 2  ->   3
 * ⁠ 3  ->   2
 * ⁠ 4  ->   5
 * ⁠ 5  ->   6
 * ⁠ 6  ->   7
 * ⁠ 7  ->   2
 * ⁠ 8  ->   5
 * ⁠ 9  ->   5
 *
 *
 * 示例 2：
 *
 *
 * 输入：cost = [7,6,5,5,5,6,8,7,8], target = 12
 * 输出："85"
 * 解释：添加数位 '8' 的成本是 7 ，添加数位 '5' 的成本是 5 。"85" 的成本为 7 + 5 = 12 。
 *
 *
 * 示例 3：
 *
 *
 * 输入：cost = [2,4,6,2,4,6,4,4,4], target = 5
 * 输出："0"
 * 解释：总成本是 target 的条件下，无法生成任何整数。
 *
 *
 * 示例 4：
 *
 *
 * 输入：cost = [6,10,15,40,40,40,40,40,40], target = 47
 * 输出："32211"
 *
 *
 *
 *
 * 提示：
 *
 *
 * cost.length == 9
 * 1
 * 1
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 动态规划(完全背包问题)
    /// 1. 为了得到最大的数字,需要如下两条:
    ///    a. 数字的位数最长;
    ///    b. 大数字尽量在高位;
    /// 2. 因此可以分两步:
    ///    a. 求出可以组成目标值的数字的最大位数;
    ///    b. 尽量将大的数位排在最大位数数字的高位;
    /// 3. 设`dp[t]`: 表示目标值为t的最大位数;
    /// 4. 则 `dp[t] = max(dp[t-ci] + 1)`
    pub fn largest_number(cost: Vec<i32>, target: i32) -> String {
        let target = target as usize;
        let mut dp = vec![i32::MIN; target + 1];
        dp[0] = 0; // target为0时, 最大整数位数为0;

        // 依次求取各个成本和的最大位数
        for &c in &cost {
            let c = c as usize;
            for t in c..=target {
                dp[t] = dp[t].max(dp[t - c] + 1);
            }
        }
        // 如果target对应的最高位数<=0, 则不存在目标和的数字
        if dp[target] <= 0 {
            return "0".into();
        }
        // 排列数位
        let mut t = target;
        let mut res = String::new();
        for (i, &c) in cost.iter().enumerate().rev() {
            let c = c as usize;
            //找到适合的数字
            while t >= c && dp[t] == dp[t - c] + 1 {
                res = format!("{}{}", res, i + 1); //将数位加入结果字符串中
                t -= c;
            }
        }
        res
    }
}
// @lc code=end
