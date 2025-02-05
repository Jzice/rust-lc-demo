/*!
 * # [122.买卖股票的最佳时机II](https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-ii/description/)
 *
 * @lc app=leetcode.cn id=122 lang=rust
 *
 * ## 难度
 * - Easy (57.53%)
 * - Likes:    613
 * - Dislikes: 0
 * - Total Accepted:    124.8K
 * - Total Submissions: 215.6K
 * - Testcase Example:  '[7,1,5,3,6,4]'
 *
 * ## 描述
 *
 * 给定一个数组，它的第 i 个元素是一支给定股票第 i 天的价格。
 *
 * 设计一个算法来计算你所能获取的最大利润。你可以尽可能地完成更多的交易（多次买卖一支股票）。
 *
 * 注意：你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。
 *
 * ## 示例 1:
 *
 * - 输入: [7,1,5,3,6,4]
 * - 输出: 7
 * - 解释: 在第 2 天（股票价格 = 1）的时候买入，在第 3 天（股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5-1 = 4 。
 * 随后，在第 4 天（股票价格 = 3）的时候买入，在第 5 天（股票价格 = 6）的时候卖出, 这笔交易所能获得利润 = 6-3 = 3 。
 *
 *
 * ## 示例 2:
 *
 * - 输入: [1,2,3,4,5]
 * - 输出: 4
 * - 解释: 在第 1 天（股票价格 = 1）的时候买入，在第 5 天 （股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5-1 = 4
 * 。
 * 注意你不能在第 1 天和第 2 天接连购买股票，之后再将它们卖出。
 * 因为这样属于同时参与了多笔交易，你必须在再次购买前出售掉之前的股票。
 *
 *
 * ## 示例 3:
 *
 * - 输入: [7,6,4,3,1]
 * - 输出: 0
 * - 解释: 在这种情况下, 没有交易完成, 所以最大利润为 0。
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路:
    /// - 动态规划+贪心
    /// 1. 设 profit[i]: 投资者从第i天开始到最后的最终投资收益;
    /// 2. 投资者每天存在2种状态:
    ///    - 未投资(uninvested):
    ///    - 已投资(invested);
    /// 3. 投资者每天可以有不同投资策略, 以获得最大利润:
    ///    - 未投资(uninvested), 此时有两种策略:
    ///       - 不操作. 当天收益: 0, 后期收益为: profit[i+1].未投资;
    ///       - 买入. 当天收益为: -prices[i], 后期收益为: profit[i+1].已投资;
    ///       最大收益为: max(0+profit[i+1].未投资, profit[i+1].已投资 - prices[i])
    ///    - 已投资(invested), 此时也有2种策略:
    ///       - 不操作. 当天收益为: 0, 后期收益为: profit[i+1].已投资;
    ///       - 卖出. 当天收益为: prices[i], 后期收益为: profit[i+1].未投资
    ///       最大收益为: max(profit[i+1].已投资, profit[i+1].未投资 + prices[i])
    /// 4. 最终最大收益为: profit[0].未投资
    ///    初始条件: profit[n]: (0, 0)  
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .rev()
            .fold((0, 0), |profit, &p| {
                (profit.0.max(-p + profit.1), profit.1.max(p + profit.0))
            })
            .0
    }

    /// ## 解题思路：
    /// [0, i]天的最大利润f(i):
    ///     f(i) = f(i-1) + max(prices[i] - prices[i-1], 0)
    pub fn max_profit1(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0 as i32;
        }
        let mut max_profit = 0;
        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                max_profit += prices[i] - prices[i - 1];
            }
        }

        max_profit
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::max_profit(vec![7,1,5,3,6,4]), 7);
        assert_eq!(Solution::max_profit(vec![1,2,3,4,5]), 4);
        assert_eq!(Solution::max_profit(vec![7,6,4,3,1]), 0);
    }
}
