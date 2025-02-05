/*
 * @lc app=leetcode.cn id=1647 lang=rust
 *
 * [1647] 字符频次唯一的最小删除次数
 *
 * https://leetcode.cn/problems/minimum-deletions-to-make-character-frequencies-unique/description/
 *
 * algorithms
 * Medium (53.76%)
 * Likes:    50
 * Dislikes: 0
 * Total Accepted:    11.1K
 * Total Submissions: 20.7K
 * Testcase Example:  '"aab"'
 *
 * 如果字符串 s 中 不存在 两个不同字符 频次 相同的情况，就称 s 是 优质字符串 。
 *
 * 给你一个字符串 s，返回使 s 成为 优质字符串 需要删除的 最小 字符数。
 *
 * 字符串中字符的 频次 是该字符在字符串中的出现次数。例如，在字符串 "aab" 中，'a' 的频次是 2，而 'b' 的频次是 1 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "aab"
 * 输出：0
 * 解释：s 已经是优质字符串。
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "aaabbbcc"
 * 输出：2
 * 解释：可以删除两个 'b' , 得到优质字符串 "aaabcc" 。
 * 另一种方式是删除一个 'b' 和一个 'c' ，得到优质字符串 "aaabbc" 。
 *
 * 示例 3：
 *
 *
 * 输入：s = "ceabaacb"
 * 输出：2
 * 解释：可以删除两个 'c' 得到优质字符串 "eabaab" 。
 * 注意，只需要关注结果字符串中仍然存在的字符。（即，频次为 0 的字符会忽略不计。）
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * s 仅含小写英文字母
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 使用数组记录每个字符出现的次数，同时用记录每个次数出现的频次
    /// 将所有频次调整到不超过1
    pub fn min_deletions(s: String) -> i32 {
        let mut cc = vec![0_usize; 26]; // 每个字符出现的次数
        let mut fq = vec![0_i16; 4096]; // 每个字符次数的频次，最终需要每个>1的频次都为1；
        let mut rm = 0_i32; // 删除操作的次数

        // 依次统计每个字符出现的次数及每个次数的频次；
        for ch in s.chars().map(|ch| ch as usize - 'a' as usize) {
            fq[cc[ch]] -= 1; // 字符计数旧频次-1；
            cc[ch] += 1; // 当前字符的计数+1；
            fq[cc[ch]] += 1; // 当前字符计数的频次+1；
        }

        // 依次每个字符次数的频次，每个频次<=1
        for mut c in cc {
            while fq[c] > 1 {
                // 当前字符计数>1时
                fq[c] -= 1; // 将频次-1
                c -= 1; // 将计数-1
                fq[c] += 1; // 新频次+1
                rm += 1; // 删除操作+1
            }
        }

        rm
    }
}
// @lc code=end
