/*
 * @lc app=leetcode.cn id=93 lang=rust
 *
 * [93] 复原 IP 地址
 *
 * https://leetcode.cn/problems/restore-ip-addresses/description/
 *
 * algorithms
 * Medium (58.08%)
 * Likes:    1252
 * Dislikes: 0
 * Total Accepted:    348.2K
 * Total Submissions: 599.2K
 * Testcase Example:  '"25525511135"'
 *
 * 有效 IP 地址 正好由四个整数（每个整数位于 0 到 255 之间组成，且不能含有前导 0），整数之间用 '.' 分隔。
 *
 *
 * 例如："0.1.2.201" 和 "192.168.1.1" 是 有效 IP 地址，但是 "0.011.255.245"、"192.168.1.312"
 * 和 "192.168@1.1" 是 无效 IP 地址。
 *
 *
 * 给定一个只包含数字的字符串 s ，用以表示一个 IP 地址，返回所有可能的有效 IP 地址，这些地址可以通过在 s 中插入 '.' 来形成。你 不能
 * 重新排序或删除 s 中的任何数字。你可以按 任何 顺序返回答案。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "25525511135"
 * 输出：["255.255.11.135","255.255.111.35"]
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "0000"
 * 输出：["0.0.0.0"]
 *
 *
 * 示例 3：
 *
 *
 * 输入：s = "101023"
 * 输出：["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= s.length <= 20
 * s 仅由数字组成
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 回溯法
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        // s是否为有效ip
        fn is_valid_ip(s: &str) -> bool {
            // 前导0无效
            if s.len() > 1 && &s[0..1] == "0" {
                return false;
            }
            // ip in [0, 256)
            match s.parse::<i32>() {
                Ok(ip) if ip >= 0 && ip < 256 => true,
                _ => false,
            }
        }

        //
        fn backtrace(s: &str, dots: &mut Vec<usize>, res: &mut Vec<String>) {
            // 终止进一步遍历条件
            if dots.len() > 2 {
                if is_valid_ip(&s[dots[2]..]) {
                    res.push(format!(
                        "{}.{}.{}.{}",
                        &s[..dots[0]].parse::<i32>().unwrap(),
                        &s[dots[0]..dots[1]].parse::<i32>().unwrap(),
                        &s[dots[1]..dots[2]].parse::<i32>().unwrap(),
                        &s[dots[2]..].parse::<i32>().unwrap()
                    ));
                }
                return;
            }
            //
            let last_dot = *dots.last().unwrap_or(&0);
            for i in last_dot + 1..s.len() {
                // 如果当前分隔为非有效ip, 则跳出当前层的试探
                if !is_valid_ip(&s[last_dot..i]) {
                    break;
                }

                // 找到一个有效的分隔点
                dots.push(i); //将其加入到有效集中

                // 递归进行下一个分隔符的试探
                backtrace(s, dots, res);

                // 撤回上一次的试探, 进行下一个试探
                dots.pop();
            }
        }

        let mut res = vec![];
        backtrace(&s, &mut vec![], &mut res);

        res
    }
}
// @lc code=end

struct Solution;
