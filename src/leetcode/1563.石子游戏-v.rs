/*!
 * # [1563.石子游戏V](https://leetcode.cn/problems/stone-game-v/description/)
 *
 * @lc app=leetcode.cn id=1563 lang=rust
 *
 * ## 难度
 * - Hard (39.38%)
 * - Likes:    48
 * - Dislikes: 0
 * - Total Accepted:    5.1K
 * - Total Submissions: 13K
 * - Testcase Example:  '[6,2,3,4,5,5]'
 *
 * ## 问题描述
 *
 * 几块石子排成一行, 每块石子都有一个关联值, 关联值为整数，由数组 stoneValue 给出。
 * 
 * 游戏中的每一轮：Alice 会将这行石子分成两个 非空行（即，左侧行和右侧行）；Bob 负责计算每一行的值，即此行中所有石子的值的总和。Bob
 * 会丢弃值最大的行，Alice 的得分为剩下那行的值（每轮累加）。如果两行的值相等，Bob 让 Alice 决定丢弃哪一行。下一轮从剩下的那一行开始。
 * 
 * 只 剩下一块石子 时，游戏结束。Alice 的分数最初为 0 。
 * 
 * 返回 Alice 能够获得的最大分数 。
 * 
 * 
 * ## 示例 1：
 * - 输入：stoneValue = [6,2,3,4,5,5]
 * - 输出：18
 * - 解释：
 *   - 在第一轮中，Alice 将行划分为 [6，2，3]，[4，5，5] 。左行的值是 11 ，右行的值是 14 。Bob 丢弃了右行，Alice的分数现在是 11 。
 *   - 在第二轮中，Alice 将行分成 [6]，[2，3] 。这一次 Bob 扔掉了左行，Alice 的分数变成了 16（11 + 5）。
 *   - 最后一轮 Alice 只能将行分成 [2]，[3] 。Bob 扔掉右行，Alice 的分数现在是 18（16 + 2）。游戏结束，因为这行只剩下一块石头了。
 * 
 * 
 * ## 示例 2：
 * - 输入：stoneValue = [7,7,7,7,7,7,7]
 * - 输出：28
 * 
 * ## 示例 3：
 * - 输入：stoneValue = [4]
 * - 输出：0
 * 
 * ## 提示：
 * - 1 <= stoneValue.length <= 500
 * - 1 <= stoneValue[i] <= 10^6
 * 
 */

use super::*;

// @lc code=start
impl Solution {
    /// ## 石子游戏V
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        todo!();
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        // assert_eq!(Solution::stone_game_v(vec![6,2,3,4,5]), 18);
        // assert_eq!(Solution::stone_game_v(vec![7,7,7,7,7,7,7]), 28);
        // assert_eq!(Solution::stone_game_v(vec![4]), 0);
    }
}
