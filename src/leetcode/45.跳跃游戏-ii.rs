/*!
 * # [45.跳跃游戏II](https://leetcode.cn/problems/jump-game-ii/description/)
 *
 * @lc app=leetcode.cn id=45 lang=rust
 *
 * ## 难度
 * Medium (43.11%)
 * Likes:    1270
 * Dislikes: 0
 * Total Accepted:    222K
 * Total Submissions: 513.5K
 * Testcase Example:  '[2,3,1,1,4]'
 *
 * ## 问题描述
 *
 * 给你一个非负整数数组 nums ，你最初位于数组的第一个位置。
 *
 * 数组中的每个元素代表你在该位置可以跳跃的最大长度。
 *
 * 你的目标是使用最少的跳跃次数到达数组的最后一个位置。
 *
 * 假设你总是可以到达数组的最后一个位置。
 *
 *
 * ## 示例 1:
 * - 输入: nums = [2,3,1,1,4]
 * - 输出: 2
 * - 解释: 跳到最后一个位置的最小跳跃数是 2。从下标为 0 跳到下标为 1 的位置，跳 1 步，然后跳 3 步到达数组的最后一个位置。
 *
 *
 * ## 示例 2:
 * - 输入: nums = [2,3,0,1,4]
 * - 输出: 2
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// # 跳跃游戏II
    /// - 贪心法
    /// 1. 依次遍历各个位置;
    /// 2. 记录每个位置时可到到达的最远位置next_max_pos;
    /// 3. 如果当前位置是上一次能跳到的最远位置,则跳到当前位置, 同时总跳的步数+1;
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut steps = 0; //已跳的步数
        let mut next_max_pos = 0; //下一步能跳到的最远位置
        let mut last_max_pos = 0; //上一次跳到的最远位置

        for pos in 0..nums.len() - 1 { //从起始位置开始一个一个条
            // 下一步可以跳到的最远的位置
            next_max_pos = next_max_pos.max(pos + nums[pos] as usize);
            // 如果可以从上一步跳最远距离到达当前位置
            if pos == last_max_pos {
                steps += 1; //跳一步
                //到达当前步能跳到的最远位置，则必须跳下一步
                last_max_pos = next_max_pos; //更新当前步最远能跳的位置
            }
        }

        steps as i32
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use crate::leetcode::_45_跳跃游戏_ii::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::jump(vec![2,3,1,1,4]), 2);
        assert_eq!(Solution::jump(vec![2,3,0,1,4]), 2);
    }
}
