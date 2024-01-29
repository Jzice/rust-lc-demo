/*!
* @lc app=leetcode.cn id=2594 lang=rust slug=minimum-time-to-repair-cars
*
* # 2594.修车的最少时间
*
* https://leetcode.cn/problems/minimum-time-to-repair-cars/description/
*
* Medium (49.29%)
*
* 给你一个整数数组 ranks ，表示一些机械工的 能力值 。ranks[i] 是第 i 位机械工的能力值** 。能力值为 r 的机械工可以在 r * n^2 分钟内修好 n 辆车。
* 同时给你一个整数 cars ，表示总共需要修理的汽车数目。
* 请你返回修理所有汽车 最少 需要多少时间。
* 注意：所有机械工可以同时修理汽车。
* 
* ## 示例 1：
* 输入：ranks = [4,2,3,1], cars = 10
* 输出：16
* 解释：
* - 第一位机械工修 2 辆车，需要 4 * 2 * 2 = 16 分钟。
* - 第二位机械工修 2 辆车，需要 2 * 2 * 2 = 8 分钟。
* - 第三位机械工修 2 辆车，需要 3 * 2 * 2 = 12 分钟。
* - 第四位机械工修 4 辆车，需要 1 * 4 * 4 = 16 分钟。
* 16 分钟是修理完所有车需要的最少时间。
* 
* ## 示例 2：
* 输入：ranks = [5,1,8], cars = 6
* 输出：16
* 解释：
* - 第一位机械工修 1 辆车，需要 5 * 1 * 1 = 5 分钟。
* - 第二位机械工修 4 辆车，需要 1 * 4 * 4 = 16 分钟。
* - 第三位机械工修 1 辆车，需要 8 * 1 * 1 = 8 分钟。
* 16 分钟时修理完所有车需要的最少时间。
* 
* ## 提示：
*
* ```text
*     1 <= ranks.length <= 105
*     1 <= ranks[i] <= 100
*     1 <= cars <= 106
* ```
*
* test case:
*
* ```text
*    [4,2,3,1]
*    10
* ```
*/

use super::*;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 二分法
    /// 1. 修车最少时间必定为[0, ranks[0]*cars^2]中的某一个时间;
    /// 2. 对于一个时间t, 每个维修工可修理的车数量为 lower(sqrt(t/ranks[i]))辆,
    ///    总可修车的数量为 c_t = sum(lower(sqrt(r/ranks[i]))),
    /// 3. 如果总可修车的数量 c_t >= cars(需要修理的汽车数量), 则说明修车需要的最少时间t_min
    ///    < t,
    ///    否则如果 c_t < cars, 则 t_min > t;
    /// 4. 因此使用二分法, 在[0, ranks[0]*cars^2]范围中查找t_min;
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let cars = cars as u64;
        if ranks.len() == 1 {
            return (ranks[0] as u64 * cars * cars) as i64;
        }
        let (mut l, mut r) = (0_u64, (ranks[0] as u64) * cars * cars);
        while l < r {
            let mid = (l + r) >> 1;
            let can_repair_cars = ranks
                .iter()
                .map(|&r| (((mid / (r as u64)) as f64).sqrt() as u64))
                .sum::<u64>();
            if can_repair_cars < cars {
                l = mid + 1;
            } else {
                r = mid;
            }
        }

        l as i64
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {}
}
