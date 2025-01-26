/*!
 * # [638.大礼包](https://leetcode.cn/problems/shopping-offers/description/)
 *
 * @lc app=leetcode.cn id=638 lang=rust
 *
 * ## 难度
 * - Medium (64.38%)
 * - Likes:    327
 * - Dislikes: 0
 * - Total Accepted:    31K
 * - Total Submissions: 48.1K
 * - Testcase Example:  '[2,5]\n[[3,0,5],[1,2,10]]\n[3,2]'
 *
 * ## 问题描述
 *
 * 在 LeetCode 商店中， 有 n 件在售的物品。每件物品都有对应的价格。然而，也有一些大礼包，每个大礼包以优惠的价格捆绑销售一组物品。
 *
 * 给你一个整数数组 price 表示物品价格，其中 price[i] 是第 i 件物品的价格。另有一个整数数组 needs 表示购物清单，其中
 * needs[i] 是需要购买第 i 件物品的数量。
 *
 * 还有一个数组 special 表示大礼包，special[i] 的长度为 n + 1 ，其中 special[i][j] 表示第 i 个大礼包中内含第
 * j 件物品的数量，且 special[i][n] （也就是数组中的最后一个整数）为第 i 个大礼包的价格。
 *
 * 返回 确切
 * 满足购物清单所需花费的最低价格，你可以充分利用大礼包的优惠活动。你不能购买超出购物清单指定数量的物品，即使那样会降低整体价格。任意大礼包可无限次购买。
 *
 * ## 示例 1：
 * - 输入：price = [2,5], special = [[3,0,5],[1,2,10]], needs = [3,2]
 * - 输出：14
 * - 解释：有 A 和 B 两种物品，价格分别为 ¥2 和 ¥5 。
 *   - 大礼包 1 ，你可以以 ¥5 的价格购买 3A 和 0B 。
 *   - 大礼包 2 ，你可以以 ¥10 的价格购买 1A 和 2B 。
 *   - 需要购买 3 个 A 和 2 个 B ， 所以付 ¥10 购买 1A 和 2B（大礼包 2），以及 ¥4 购买 2A 。
 *
 * ## 示例 2：
 * - 输入：price = [2,3,4], special = [[1,1,0,4],[2,2,1,9]], needs = [1,2,1]
 * - 输出：11
 * - 解释：A ，B ，C 的价格分别为 ¥2 ，¥3 ，¥4 。
 * 可以用 ¥4 购买 1A 和 1B ，也可以用 ¥9 购买 2A ，2B 和 1C 。
 * 需要买 1A ，2B 和 1C ，所以付 ¥4 买 1A 和 1B（大礼包 1），以及 ¥3 购买 1B ， ¥4 购买 1C 。
 * 不可以购买超出待购清单的物品，尽管购买大礼包 2 更加便宜。
 *
 * ## 提示：
 * - n == price.length
 * - n == needs.length
 * - special[i].length == n + 1
 *
 */

struct Solution;

// @lc code=start
use std::collections::HashMap;

impl Solution {
    /// ## 解题思路
    /// * 动态规划
    ///
    /// 计算满足购物清单所需花费的最低价格
    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        let mut memo = HashMap::new(); // 初始化记忆化哈希表
        Self::dfs(&price, &special, &needs, &mut memo) // 调用辅助函数计算最低价格
    }

    /// 递归辅助函数，计算满足当前需求的最低价格
    fn dfs(price: &Vec<i32>, special: &Vec<Vec<i32>>, needs: &Vec<i32>, memo: &mut HashMap<Vec<i32>, i32>) -> i32 {
        // 如果当前需求已经计算过，直接返回存储的结果
        if let Some(&res) = memo.get(needs) {
            return res;
        }

        // 计算不使用大礼包时的总价格
        let mut res = 0;
        for (i, &need) in needs.iter().enumerate() {
            res += need * price[i];
        }

        // 尝试使用每个大礼包
        for sp in special {
            let mut clone = needs.clone(); // 复制当前需求
            let mut valid = true; // 标记大礼包是否有效
            for i in 0..needs.len() {
                if clone[i] < sp[i] { // 如果大礼包中的物品数量超过当前需求
                    valid = false; // 标记为无效
                    break;
                }
                clone[i] -= sp[i]; // 更新需求
            }
            if valid {
                // 递归计算使用大礼包后的最低价格，并更新结果
                res = res.min(sp[needs.len()] + Self::dfs(price, special, &clone, memo));
            }
        }

        // 存储当前需求的最低价格
        memo.insert(needs.clone(), res);
        res // 返回最低价格
    }
}
// @lc code=end

use super::*;
