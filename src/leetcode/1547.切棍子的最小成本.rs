/*!
 * # [1547.切棍子的最小成本](https://leetcode.cn/problems/minimum-cost-to-cut-a-stick/description/)
 * 
 * @lc app=leetcode.cn id=1547 lang=rust
 *
 * ## 难度
 *
 * - Hard (55.98%)
 * - Total Accepted:    5.4K
 * - Total Submissions: 9.7K
 * - Testcase Example:  '7\n[1,3,4,5]'
 *
 * ## 描述
 *
 * 有一根长度为 n 个单位的木棍，棍上从 0 到 n 标记了若干位置。例如，长度为 6 的棍子可以标记如下：
 *
 *
 * 给你一个整数数组 cuts ，其中 cuts[i] 表示你需要将棍子切开的位置。
 *
 * 你可以按顺序完成切割，也可以根据需要更改切割的顺序。
 *
 *
 * 每次切割的成本都是当前要切割的棍子的长度，切棍子的总成本是历次切割成本的总和。对棍子进行切割将会把一根木棍分成两根较小的木棍（这两根木棍的长度和就是切割前木棍的长度）。请参阅第一个示例以获得更直观的解释。
 *
 * 返回切棍子的 最小总成本 。
 *
 *
 *
 * ## 示例 1：
 *
 * - 输入：n = 7, cuts = [1,3,4,5]
 * - 输出：16
 * - 解释：按 [1, 3, 4, 5] 的顺序切割的情况如下所示：
 *
 * 第一次切割长度为 7 的棍子，成本为 7 。第二次切割长度为 6 的棍子（即第一次切割得到的第二根棍子），第三次切割为长度 4 的棍子，最后切割长度为3 的棍子。总成本为 7 + 6 + 4 + 3 = 20 。
 *
 * 而将切割顺序重新排列为 [3, 5, 1, 4] 后，总成本 = 16（如示例图中 7 + 4 + 3 + 2 = 16）。
 *
 *
 * ## 示例 2：
 * - 输入：n = 9, cuts = [5,6,1,4,2]
 * - 输出：22
 * - 解释：如果按给定的顺序切割，则总成本为 25 。总成本
 *
 *
 * ## 提示：
 * - cuts 数组中的所有整数都 互不相同
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
        todo!()
    }
}

// @lc code=end


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::min_cost(7, vec![1,3,4,5]), 16);
        assert_eq!(Solution::min_cost(9, vec![5,6,1,4,2]), 22);
    }
}
