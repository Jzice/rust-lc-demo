/*!
 * # [57.插入区间](https://leetcode.cn/problems/insert-interval/description/)
 *
 * @lc app=leetcode.cn id=57 lang=rust
 *
 * ## 难度
 *
 * - Medium (41.90%)
 * - Likes:    704
 * - Dislikes: 0
 * - Total Accepted:    143.5K
 * - Total Submissions: 342.4K
 * - Testcase Example:  '[[1,3],[6,9]]\n[2,5]'
 *
 * ## 题目描述
 *
 * 给你一个 无重叠的 ，按照区间起始端点排序的区间列表。
 *
 * 在列表中插入一个新的区间，你需要确保列表中的区间仍然有序且不重叠（如果有必要的话，可以合并区间）。
 *
 *
 * ## 示例 1：
 *
 * - 输入：intervals = [[1,3],[6,9]], newInterval = [2,5]
 * - 输出：[[1,5],[6,9]]
 *
 *
 * ## 示例 2：
 *
 * - 输入：intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
 * - 输出：[[1,2],[3,10],[12,16]]
 * - 解释：这是因为新的区间 [4,8] 与 [3,5],[6,7],[8,10] 重叠。
 *
 * ## 示例 3：
 *
 * - 输入：intervals = [], newInterval = [5,7]
 * - 输出：[[5,7]]
 *
 *
 * ## 示例 4：
 *
 * - 输入：intervals = [[1,5]], newInterval = [2,3]
 * - 输出：[[1,5]]
 *
 *
 * ## 示例 5：
 *
 * - 输入：intervals = [[1,5]], newInterval = [2,7]
 * - 输出：[[1,7]]
 *
 *
 *
 *
 * # 提示：
 *
 * - intervals[i].length == 2
 * - intervals 根据 intervals[i][0] 按 升序 排列
 * - newInterval.length == 2
 *
 *
 */

use super::*;

// @lc code=start
impl Solution {
    /// # 插入区间
    /// ## 解题思路
    /// - new_interval将intervals划分为三个部分:
    ///   1. new_interval左侧(val[1]<new_interval[0]), 此部分都在一开始push到结果集res中;
    ///   2. new_interval右侧, 此部分在之前部分合并后都直接push到res;
    ///   3. new_interval和vals有重叠, 此时为一个重叠区间,区间左右边界是[min(new_interval[0], val[0]), max(new_interval[1], val[1])];
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut overlay_interval = new_interval;
        let mut placed = false;

        for val in intervals {
            if val[1] < overlay_interval[0] {
                //new_interval左侧
                res.push(val);
            } else if val[0] > overlay_interval[1] {
                //new_interval右侧
                if !placed {
                    //先将合并区间插入
                    res.push(vec![overlay_interval[0], overlay_interval[1]]);
                    placed = true;
                }
                res.push(val);
            } else {
                //重叠区间
                overlay_interval[0] = overlay_interval[0].min(val[0]);
                overlay_interval[1] = overlay_interval[1].max(val[1]);
            }
        }
        if !placed {
            res.push(vec![overlay_interval[0], overlay_interval[1]]);
            // placed = true;
        }

        res
    }
}
// @lc code=end
