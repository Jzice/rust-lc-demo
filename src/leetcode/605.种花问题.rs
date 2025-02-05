/*!
 * # [605.种花问题](https://leetcode.cn/problems/can-place-flowers/description/)
 *
 * @lc app=leetcode.cn id=605 lang=rust
 *
 * ## 难度
 * - Easy (33.16%)
 * - Likes:    435
 * - Dislikes: 0
 * - Total Accepted:    124.4K
 * - Total Submissions: 375.6K
 * - Testcase Example:  '[1,0,0,0,1]\n1'
 *
 * ## 问题描述
 *
 * 假设有一个很长的花坛，一部分地块种植了花，另一部分却没有。可是，花不能种植在相邻的地块上，它们会争夺水源，两者都会死去。
 *
 * 给你一个整数数组  flowerbed 表示花坛，由若干 0 和 1 组成，其中 0 表示没种植花，1 表示种植了花。另有一个数 n
 * ，能否在不打破种植规则的情况下种入 n 朵花？能则返回 true ，不能则返回 false。
 *
 *
 * ## 示例 1：
 * - 输入：flowerbed = [1,0,0,0,1], n = 1
 * - 输出：true
 *
 *
 * ## 示例 2：
 * - 输入：flowerbed = [1,0,0,0,1], n = 2
 * - 输出：false
 *
 * ## 提示：
 * - flowerbed[i] 为 0 或 1
 * - flowerbed 中不存在相邻的两朵花
 *
 */

// @lc code=start
impl Solution {
    /// ## 种花问题
    /// - 贪心法
    /// 1. 从左到右遍历数组;
    /// 2. 如果当前位置为1, 则表明当前位置已经有花了, 跳2格到下下一个位置;
    /// 3. 否则当前位置为0.
    /// 4. 由于不可能有位置相邻的1, 且当前面每次遇到1时, 必须跳两格, 所以此时只需判断后面是否为0;
    /// 5. 如果f[i+1] == 0 或者 i=len-1, 则可以在当前位置种花, 种完后跳2格;
    /// 6. 如果f[i+1] == 1, 则其下一格f[i+2]必定不能种, 需要往后跳3格;
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        if n <= 0 {
            return true;
        }
        let mut n = n;
        let mut i = 0;
        while i < flowerbed.len() && n > 0 {
            if flowerbed[i] == 1 {
                i += 2;
            } else if i == flowerbed.len() - 1 || flowerbed[i + 1] == 0 {
                n -= 1;
                i += 2;
            } else {
                i += 3;
            }
        }

        return n <= 0;
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::can_place_flowers(vec![1, 0, 0], 1), true);
        assert_eq!(
            Solution::can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2),
            false
        );
        assert_eq!(
            Solution::can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2),
            false
        );
    }
}
