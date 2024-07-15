/*!
 * # [887.鸡蛋掉落](https://leetcode.cn/problems/super-egg-drop/description/)
 * 
 * @lc app=leetcode.cn id=887 lang=rust
 *
 * ## 难度
 * - Hard (30.83%)
 * - Likes:    985
 * - Dislikes: 0
 * - Total Accepted:    82.3K
 * - Total Submissions: 266.4K
 * - Testcase Example:  '1\n2'
 *
 * ## 描述
 *
 * 给你 k 枚相同的鸡蛋，并可以使用一栋从第 1 层到第 n 层共有 n 层楼的建筑。
 * 
 * 已知存在楼层 f ，满足 `0 <= f <= n` ，任何从 高于 f 的楼层落下的鸡蛋都会碎，从 f 楼层或比它低的楼层落下的鸡蛋都不会破。
 * 
 * 每次操作，你可以取一枚没有碎的鸡蛋并把它从任一楼层 x 扔下（满足 `1 <= x <= n`）。如果鸡蛋碎了，你就不能再次使用它。如果某枚鸡蛋扔下后没有摔碎，则可以在之后的操作中 重复使用 这枚鸡蛋。
 * 
 * 请你计算并返回要确定 f 确切的值 的 最小操作次数 是多少？
 * 
 * 
 * ## 示例 1：
 * - 输入：k = 1, n = 2
 * - 输出：2
 * - 解释：
 *   - 鸡蛋从 1 楼掉落。如果它碎了，肯定能得出 f = 0 。 
 *   - 否则，鸡蛋从 2 楼掉落。如果它碎了，肯定能得出 f = 1 。 
 *   - 如果它没碎，那么肯定能得出 f = 2 。 
 *   - 因此，在最坏的情况下我们需要移动 2 次以确定 f 是多少。 
 * 
 * ## 示例 2：
 * - 输入：k = 2, n = 6
 * - 输出：3
 * 
 * ## 示例 3：
 * - 输入：k = 3, n = 14
 * - 输出：4
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 鸡蛋掉落
    /// - 动态规划1(超时)
    /// 1. 设 dp[i][j] 表示总共有i个鸡蛋, 总共j层楼时, 确定目标楼层f的最小操作次数;
    /// 2. 如果最开始选择在x楼(1<=x<=j)扔1个鸡蛋, 此时会有两种情况：
    ///    a. 鸡蛋未破, 则目标楼层一定不在`[1,x)`中, 此时可继续用i个鸡蛋在`(x,n]`楼层中尝试.
    ///       后续为确定目标楼层的最小操作次数为: dp[i][j-x]
    ///    b. 鸡蛋破裂, 则目标楼层一定不在`(x,j]`中, 此时可用剩下的i-1个鸡蛋在`[1,x)`楼层中继续尝试.
    ///       后续为确定目标楼层的最小操作次数为: dp[i-1][x-1]
    ///    综合以上情况, 如果扔1枚鸡蛋在x楼后, 为保证一定能确定目标楼层, 后续最小操作次数为: 
    ///       max(dp[i][j-x], dp[i-1][x-1])
    ///    那么, 依次从`[1,j]`中选择开始楼层x扔鸡蛋, 选择总操作数最小的即为dp[i][j]:
    ///       min(max(dp[i][j-x], dp[i-1][x-1]) + 1)  ( 1 <= x <= j )
    /// 3. 综上可得递推关系:
    ///       dp[i][j] = 1 + min(max(dp[i][j-k], dp[i-1][k-1])) ( 1 <= k <= j  ) 
    /// 4. 初始条件: 当只有1个鸡蛋时, 需要从低到高逐层进行尝试, 最坏情况需要到最高一层n才能确定,
    ///    因此: dp[1][j] = j ( 1 <= j <= n )
    /// 5. 目标值: dp[k][n]
    pub fn super_egg_drop1(k: i32, n: i32) -> i32 {
        let (k, n) = (k as usize, n as usize);
        let mut dp = vec![vec![0; n as usize +1]; k as usize + 1];
        // 初始条件
        for j in 1..=n {
            dp[1][j] = j;
        }
        for i in 2..=k {
            for j in 2..=n {
                dp[i][j] = 1 + (1..=j)
                    .map(|x|{
                        dp[i][j-x].max(dp[i-1][x-1])
                    })
                    .min()
                    .unwrap_or(1);
            }
        }

        dp[k][n] as i32
    }

    /// ## 鸡蛋掉落
    /// - 动态规划2
    /// 1. 设 dp[i][j] 表示: 总共有i个鸡蛋, 扔鸡蛋j次, 最多可检测的楼层数;
    /// 2. 扔1次鸡蛋后(不论扔在那个楼层), 有两种情况:
    ///    a. 鸡蛋未破, 剩余i个鸡蛋, 剩余操作次数j-1, 最多检测楼层数为: dp[i][j-1]
    ///    b. 鸡蛋破了, 剩余i-1个鸡蛋, 剩余操作次数j-1, 最多可检测楼层数为: dp[i-1][j-1]
    ///    那么, 最多可检测楼层总数: 
    ///       dp[i][j] = 1 + dp[i][j-1] + dp[i-1][j-1]
    /// 3. 初始条件:
    ///    a. 鸡蛋个数i: 1 <= i <= k
    ///    a. 总操作次数j最多为楼层数: 1 <= j <= n;
    ///    b. 1个鸡蛋1次最多可检测楼层数为1: dp[1][1] = 1;
    /// 4. 目标:
    ///    i, j 由小到大, 当: i == k && dp[i][j] >= n时, j的值
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let (k, n) = (k as usize, n as usize);
        let mut dp = vec![vec![0; n + 1]; k + 1];
        let mut j = 0; // 操作次数
        while dp[k][j] < n {
            j += 1;
            for i in 1..=k {
                dp[i][j] = 1 + dp[i][j-1] + dp[i-1][j-1];
            }
        }

        j as i32
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::super_egg_drop(1, 2), 2);
        assert_eq!(Solution::super_egg_drop(2, 6), 3);
        assert_eq!(Solution::super_egg_drop(3, 14), 4);
    }
}
