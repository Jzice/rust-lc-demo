/*
 * @lc app=leetcode.cn id=821 lang=rust
 *
 * [821] 字符的最短距离
 *
 * https://leetcode.cn/problems/shortest-distance-to-a-character/description/
 *
 * algorithms
 * Easy (72.93%)
 * Likes:    332
 * Dislikes: 0
 * Total Accepted:    72.9K
 * Total Submissions: 100K
 * Testcase Example:  '"loveleetcode"\n"e"'
 *
 * 给你一个字符串 s 和一个字符 c ，且 c 是 s 中出现过的字符。
 *
 * 返回一个整数数组 answer ，其中 answer.length == s.length 且 answer[i] 是 s 中从下标 i 到离它 最近
 * 的字符 c 的 距离 。
 *
 * 两个下标 i 和 j 之间的 距离 为 abs(i - j) ，其中 abs 是绝对值函数。
 *
 *
 *
 * ## 示例 1：
 *
 *
 * 输入：s = "loveleetcode", c = "e"
 * 输出：[3,2,1,0,1,0,0,1,2,2,1,0]
 * 解释：字符 'e' 出现在下标 3、5、6 和 11 处（下标从 0 开始计数）。
 * 距下标 0 最近的 'e' 出现在下标 3 ，所以距离为 abs(0 - 3) = 3 。
 * 距下标 1 最近的 'e' 出现在下标 3 ，所以距离为 abs(1 - 3) = 2 。
 * 对于下标 4 ，出现在下标 3 和下标 5 处的 'e' 都离它最近，但距离是一样的 abs(4 - 3) == abs(4 - 5) = 1 。
 * 距下标 8 最近的 'e' 出现在下标 6 ，所以距离为 abs(8 - 6) = 2 。
 *
 *
 * ## 示例 2：
 *
 *
 * 输入：s = "aaab", c = "b"
 * 输出：[3,2,1,0]
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= s.length <= 10^4
 * s[i] 和 c 均为小写英文字母
 * 题目数据保证 c 在 s 中至少出现一次
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 正反遍历
    /// 1. 任意一个字符到最近c的距离为其到左c和右c距离中小的那个;
    /// 2. 为此可正反各遍历一次,取得各个字符相对于其左c和右c的距离,最后去其中较小的;
    /// 3. 注意初始左右下标的设置;
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut res = vec![0; s.len()]; // 结果数组

        let mut left_c = -(s.len() as i32); //初始left c的下标应该离最左边下标>=n, 故至少为-s.len()
        for (i, sc) in s.chars().enumerate() { //自左向右遍历字符串
            if sc == c {                //如果当前字符为c
                left_c = i as i32;      //更新left_c
            }
            res[i] = i as i32 - left_c; //计算当前字母到最近left_c的距离
        }

        let mut right_c = 2 * s.len(); //初始right c的下标离最右边的元素下标也应该>=n,
                                       //故可设置为2*s.len()
        for (i, sc) in s.chars().rev().enumerate() { //自右向左遍历字符串
            let i = s.len() - 1 - i; //转换为正常的index
            if sc == c {            //如果当前字符为c
                right_c = i;        //记录right_c
            }
            res[i] = res[i].min((right_c - i) as i32); //计算当前字符到最近right_c的距离,并计算和left_c距离两者中的更小值
        }

        res
    }
}
// @lc code=end

struct Solution;
