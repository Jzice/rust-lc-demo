/*
 * @lc app=leetcode.cn id=123 lang=rust
 *
 * [123] 买卖股票的最佳时机 III
 *
 * https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-iii/description/
 *
 * algorithms
 * Hard (41.21%)
 * Likes:    308
 * Dislikes: 0
 * Total Accepted:    24.3K
 * Total Submissions: 58.3K
 * Testcase Example:  '[3,3,5,0,0,3,1,4]'
 *
 * 给定一个数组，它的第 i 个元素是一支给定的股票在第 i 天的价格。
 *
 * 设计一个算法来计算你所能获取的最大利润。你最多可以完成 两笔 交易。
 *
 * 注意: 你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。
 *
 * 示例 1:
 *
 * 输入: [3,3,5,0,0,3,1,4]
 * 输出: 6
 * 解释: 在第 4 天（股票价格 = 0）的时候买入，在第 6 天（股票价格 = 3）的时候卖出，这笔交易所能获得利润 = 3-0 = 3 。
 * 随后，在第 7 天（股票价格 = 1）的时候买入，在第 8 天 （股票价格 = 4）的时候卖出，这笔交易所能获得利润 = 4-1 = 3 。
 *
 * 示例 2:
 *
 * 输入: [1,2,3,4,5]
 * 输出: 4
 * 解释: 在第 1 天（股票价格 = 1）的时候买入，在第 5 天 （股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5-1 = 4
 * 。
 * 注意你不能在第 1 天和第 2 天接连购买股票，之后再将它们卖出。
 * 因为这样属于同时参与了多笔交易，你必须在再次购买前出售掉之前的股票。
 *
 *
 * 示例 3:
 *
 * 输入: [7,6,4,3,1]
 * 输出: 0
 * 解释: 在这个情况下, 没有交易完成, 所以最大利润为 0。
 *
 */

struct Solution;

// @lc code=start
use std::cmp;

impl Solution {
    /// ## 解题思路:
    /// - 动态规划+贪心
    /// 1. 设 profit[i]: 投资者从第i天开始到最后的最终投资收益;
    /// 2. 投资者每天存在2种状态:
    ///    - 未投资(uninvested):
    ///    - 已投资(invested);
    /// 3. 投资者每天可以有不同投资策略, 以获得最大利润:
    ///    - 未投资(uninvested), 此时有两种投资策略:
    ///       - 不操作. 此时当天投资收益为0, 后续投资收益为: profit[i+1].未投资;
    ///       - 买入. 此时当天投资收益为: -prices[i], 后续投资收益为: profit[i+1].已投资;
    ///       最大收益为: max(0+profit[i+1].未投资, profit[i+1].已投资 - prices[i])
    ///    - 已投资(invested), 此时也有2总策略:
    ///       - 不操作. 此时当天投资收益为: 0, 后续投资收益为: profit[i+1].已投资;
    ///       - 卖出. 此时当天投资收益为: prices[i]. 后续投资收益为: profit[i+1].未投资
    ///       最大收益为: max(profit[i+1].已投资, profit[i+1].未投资 + prices[i])
    /// 4. 最终最大收益为: profit[0].未投资
    ///    初始条件: profit[n]: (0, 0)  
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let K = 2;
        let n = prices.len();
        let mut profit = vec![(0, 0); K + 1];
        for &p in prices.iter().rev() {
            for k in 1..=K {
                profit[k].0 = profit[k].0.max(-p + profit[k].1);
                profit[k].1 = profit[k].1.max(p + profit[k - 1].0);
            }
        }
        profit[K].0
    }

    /// ## 解题思路：
    /// 第i天，共有5种操作：
    /// 1. 无操作；
    /// 2. 第1次买入；
    /// 3. 第1次卖出；
    /// 4. 第2次买入；
    /// 5. 第2次卖出；
    ///
    /// 其中第1种操作不会产生任何利润，后面4次操作最大利润递推公式分别如下：
    /// 1. buy1(i) = max(buy1(i-1), -prices[i])， 注意: buy1(i-1)：表示[0, i-1]执行了一次买入操作的最大利润, 则第i天将不执行买入操作；-prices[i]表示第i天执行买入操作，所以利润为负；
    /// 2. sell1(i) = max(sell1(i-1), buy1(i-1)+prices[i]), 第i天第1次卖出操作的最大利润为：不操作，此时最大利润为前一天第一次卖出最大利润或第一次卖出，最大利润和
    /// 3. buy2(i) = max(buy2(i-1), sell1(i-1)-prices[i])
    /// 3. sell2(i) = max(sell2(i-1), buy2(i-1)+prices[i])
    pub fn max_profit2(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0 as i32;
        }
        let mut buy1 = 0 - prices[0]; // 第一次买入后的收益(第一次买入前手中没有股票, 也没有参与过买卖,)
        let mut sell1 = 0; // 第一次卖出后的收益(卖出前手中的股票为0)
        let mut buy2 = -prices[0]; // 第二次买入后的收益
        let mut sell2 = 0; // 第二次卖出后的收益
        for i in 1..prices.len() {
            buy1 = cmp::max(buy1, -prices[i]);
            sell1 = cmp::max(sell1, buy1 + prices[i]);
            buy2 = cmp::max(buy2, sell1 - prices[i]);
            sell2 = cmp::max(sell2, buy2 + prices[i]);
        }

        sell2
    }
}
// @lc code=end
