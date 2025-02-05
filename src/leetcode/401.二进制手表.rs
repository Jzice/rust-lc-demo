/*
 * @lc app=leetcode.cn id=401 lang=rust
 *
 * [401] 二进制手表
 *
 * https://leetcode.cn/problems/binary-watch/description/
 *
 * algorithms
 * Easy (61.77%)
 * Likes:    363
 * Dislikes: 0
 * Total Accepted:    61.2K
 * Total Submissions: 98.9K
 * Testcase Example:  '1'
 *
 * 二进制手表顶部有 4 个 LED 代表 小时（0-11），底部的 6 个 LED 代表 分钟（0-59）。每个 LED 代表一个 0 或
 * 1，最低位在右侧。
 * 
 * 
 * 例如，下面的二进制手表读取 "3:25" 。
 * 
 * 
 * 
 * 
 * （图源：WikiMedia - Binary clock samui moon.jpg ，许可协议：Attribution-ShareAlike 3.0
 * Unported (CC BY-SA 3.0) ）
 * 
 * 给你一个整数 turnedOn ，表示当前亮着的 LED 的数量，返回二进制手表可以表示的所有可能时间。你可以 按任意顺序 返回答案。
 * 
 * 小时不会以零开头：
 * 
 * 
 * 例如，"01:00" 是无效的时间，正确的写法应该是 "1:00" 。
 * 
 * 
 * 分钟必须由两位数组成，可能会以零开头：
 * 
 * 
 * 例如，"10:2" 是无效的时间，正确的写法应该是 "10:02" 。
 * 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：turnedOn = 1
 * 输出：["0:01","0:02","0:04","0:08","0:16","0:32","1:00","2:00","4:00","8:00"]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：turnedOn = 9
 * 输出：[]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 0 
 * 
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// * hour: [0-11], minutes: [0-59]
    /// * 分别枚举h, m, 统计其中1的个数和；
    /// * 如果和为turned_on，则将h:m加入到结果数组中；
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut ans: Vec<String> = vec!();
        for h in 0..12_u8 {
            for m in 0..60_u8 {
                if h.count_ones() + m.count_ones() == turned_on as u32 {
                    if m < 10 {
                        ans.push(format!("{}:0{}", h, m));
                    } else {
                        ans.push(format!("{}:{}", h, m));
                    }
                }
            }
        }

        ans
    }
}
// @lc code=end

