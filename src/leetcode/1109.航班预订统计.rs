/*!
 * # [1109.航班预订统计](https://leetcode.cn/problems/corporate-flight-bookings/description/)
 *
 * @lc app=leetcode.cn id=1109 lang=rust
 *
 * ## 难度
 * - Medium (64.30%)
 * - Likes:    509
 * - Dislikes: 0
 * - Total Accepted:    120.2K
 * - Total Submissions: 187K
 * - Testcase Example:  '[[1,2,10],[2,3,20],[2,5,25]]\n5'
 *
 * ## 描述
 *
 * 这里有 n 个航班，它们分别从 1 到 n 进行编号。
 *
 * 有一份航班预订表 bookings ，表中第 i 条预订记录 bookings[i] = [firsti, lasti, seatsi] 意味着在从
 * firsti 到 lasti （包含 firsti 和 lasti ）的 每个航班 上预订了 seatsi 个座位。
 *
 * 请你返回一个长度为 n 的数组 answer，里面的元素是每个航班预定的座位总数。
 *
 *
 *
 * ## 示例 1：
 * - 输入：bookings = [[1,2,10],[2,3,20],[2,5,25]], n = 5
 * - 输出：[10,55,45,25,25]
 * - 解释：
 * ```text
 * 航班编号        1   2   3   4   5
 * 预订记录 1 ：   10  10
 * 预订记录 2 ：       20  20
 * 预订记录 3 ：       25  25  25  25
 * 总座位数：      10  55  45  25  25
 * 因此，answer = [10,55,45,25,25]
 * ```
 *
 * ## 示例 2：
 * - 输入：bookings = [[1,2,10],[2,2,15]], n = 2
 * - 输出：[10,25]
 * - 解释：
 * ```text
 * 航班编号        1   2
 * 预订记录 1 ：   10  10
 * 预订记录 2 ：       15
 * 总座位数：      10  25
 * 因此，answer = [10,25]
 * ```
 *
 * ## 提示：
 * - 1 <= n <= 2 * 10^4
 * - 1 <= bookings.length <= 2 * 10^4
 * - bookings[i].length == 3
 * - 1 <= firsti <= lasti <= n
 * - 1 <= seatsi <= 10^4
 *
 *
 */

use super::*;

// @lc code=start
impl Solution {
    /// # 航班预订统计
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut res = vec![0; n as usize];
        for b in bookings {
            for bi in b[0]..=b[1] {
                res[bi as usize - 1] += b[2];
            }
        }

        res
    }
}
// @lc code=end
//
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::corp_flight_bookings(vec![vec![1,2,10],vec![2,3,20],vec![2,5,25]], 5), vec![10,55,45,25,25]);
        assert_eq!(Solution::corp_flight_bookings(vec![vec![1,2,10],vec![2,2,15]], 2), vec![10,25]);
    }
}
