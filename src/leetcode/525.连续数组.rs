/*!
 * # [525.连续数组](https://leetcode.cn/problems/contiguous-array/description/)
 * 
 * @lc app=leetcode.cn id=525 lang=rust
 *
 * ## 难度
 * - Medium (54.72%)
 * - Likes:    671
 * - Dislikes: 0
 * - Total Accepted:    70K
 * - Total Submissions: 128K
 * - Testcase Example:  '[0,1]'
 * 
 * ## 问题描述
 *
 * 给定一个二进制数组 nums , 找到含有相同数量的 0 和 1 的最长连续子数组，并返回该子数组的长度。
 *
 * ## 示例 1:
 * - 输入: nums = [0,1]
 * - 输出: 2
 * - 说明: [0, 1] 是具有相同数量 0 和 1 的最长连续子数组。
 *
 * ## 示例 2:
 * - 输入: nums = [0,1,0]
 * - 输出: 2
 * - 说明: [0, 1] (或 [1, 0]) 是具有相同数量0和1的最长连续子数组。
 *
 * ## 提示：
 * - nums[i] 不是 0 就是 1
 */
struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 前缀差+hashmap
    /// 1. 设置 diff: 表示当前位置0,1次数的差值;
    /// 2. 设置 hashmap, 记录最近一次(diff, i);
    /// 3. 从左至右依次遍历序列, 如果为0, 则diff+=1, 否则, diff-=1;
    /// 4. 如果 diff == 0, 则说明从开始到当前位置的连续子数组中0,1出现次数相等, 更新最大子数组长度;
    /// 5. 否则diff不为0, 检查hashmap中当前diff是否有记录
    ///    如果不存在, 则将当前(diff, i)记录到hashmap中;
    /// 6. 如果存在, 则表明当前位置i到hashmap中记录的上一个当前diff位置last_i之间的连续子数组
    ///    0,1出现次数相等, 更新最大连续子数组长度;
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        let mut max_len = 0;
        let mut diff = 0; // 当前位置0,1的次数差

        // 遍历nums
        for i in 0..nums.len() {
            // 计算差值diff
            match nums[i] {
                0 => diff += 1,
                _ => diff -= 1,
            }

            // 如果diff为0, 则表示从0开始到当前位置的子数组0,1相等
            if diff == 0 {
                // 更新最大子数组长度
                max_len = max_len.max((i as i32) + 1);
            } else if !map.contains_key(&diff) {
                // 否则, 如果当前差值diff在hashmap没有记录,
                // 则记录到hashmap中
                map.insert(diff, i);
            } else {
                // 否则, 如果当前差值在之前出现过(hashmap中记录了)
                // 则当前位置i到hashmap中记录值之间的子数组的0,1出现的数量必定相等,
                // 更新最大子数组长度
                max_len = max_len.max((i - map.get(&diff).unwrap()) as i32);
            }
        }

        max_len
    }
}
// @lc code=end
