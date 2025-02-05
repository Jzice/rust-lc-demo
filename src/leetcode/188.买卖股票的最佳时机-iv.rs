/*!
 * # [188.买卖股票的最佳时机IV](https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-iv/description/)
 *
 * @lc app=leetcode.cn id=188 lang=rust
 *
 * ## 难度
 * - Hard (45.56%)
 * - Likes:    972
 * - Dislikes: 0
 * - Total Accepted:    194K
 * - Total Submissions: 424.1K
 * - Testcase Example:  '2\n[2,4,1]'
 *
 * ## 描述
 *
 * 给定一个整数数组 prices ，它的第 i 个元素 prices[i] 是一支给定的股票在第 i 天的价格，和一个整型 k 。
 *
 * 设计一个算法来计算你所能获取的最大利润。你最多可以完成 k 笔交易。也就是说，你最多可以买 k 次，卖 k 次。
 *
 * 注意：你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。
 *
 * ## 示例 1：
 * - 输入：k = 2, prices = [2,4,1]
 * - 输出：2
 * - 解释：在第 1 天 (股票价格 = 2) 的时候买入，在第 2 天 (股票价格 = 4) 的时候卖出，这笔交易所能获得利润 = 4-2 = 2 。
 *
 * ## 示例 2：
 * - 输入：k = 2, prices = [3,2,6,5,0,3]
 * - 输出：7
 * - 解释：在第 2 天 (股票价格 = 2) 的时候买入，在第 3 天 (股票价格 = 6) 的时候卖出, 这笔交易所能获得利润 = 6-2 = 4 。
 * 随后，在第 5 天 (股票价格 = 0) 的时候买入，在第 6 天 (股票价格 = 3) 的时候卖出, 这笔交易所能获得利润 = 3-0 = 3 。
 *
 * ## 提示：
 * - 0 <= k <= 100
 * - 0 <= prices.length <= 1000
 * - 0 <= prices[i] <= 1000
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 买卖股票的最佳时机IV
    /// - 动态规划
    /// 1. 设 profit[i]: 投资者第i天的收益;
    /// 2. 投资者有以下几个状态:
    ///    - 未投资. 此时投资者有如下选择:
    ///      - 不操作, 当天收益为: 0, 后续收益为: profit[i+1][k].未投资;
    ///      - 买入, 当天收益为: -prices[i], 后续收益为: profit[i+1][k].已投资;
    ///      所以最终最大收益为:
    ///        profit[i][k].未投资 = max(0+profit[i+1][k].未投资, -prices[i]+profit[i+1][k].已投资)
    ///    - 已投资. 此时投资者有如下选择:
    ///      - 不操作, 当天收益为: 0, 后续收益为: profit[i+1][k].已投资
    ///      - 卖出, 当天收益为: prices[i], 后续收益为: profit[i+1][k-1].未投资
    ///      所以最终最大收益为:
    ///        profit[i][k].已投资 = max(0+profit[i+1][k].已投资, -prices[i]+profit[i+1][k-1].未投资)
    ///  3. 最终最大收益: profit[0][k].0
    ///     初始条件: profit[n][0] = (0, 0)
    ///  4. 优化:
    ///     profit[i][] 只和 profit[i+1], prices[i] 相关, 使用单一变量来代替数组;
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        let mut profit = vec![(0, 0); k + 1];
        for &p in prices.iter().rev() {
            for j in 1..=k {
                profit[j].0 = profit[k].0.max(-p + profit[j].1);
                profit[j].1 = profit[k].1.max(p + profit[j - 1].0);
            }
        }
        profit[k].0
    }
}
// @lc code=end

