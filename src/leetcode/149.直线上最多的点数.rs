/*
 * # [149.直线上最多的点数](https://leetcode.cn/problems/max-points-on-a-line/description/)
 *
 * @lc app=leetcode.cn id=149 lang=rust
 *
 * ## 难度
 *
 * - Hard (39.34%)
 * - Likes:    505
 * - Dislikes: 0
 * - Total Accepted:    81.2K
 * - Total Submissions: 206K
 * - Testcase Example:  '[[1,1],[2,2],[3,3]]'
 *
 * ## 问题描述
 *
 * 给你一个数组 points ，其中 points[i] = [xi, yi] 表示 X-Y 平面上的一个点。求最多有多少个点在同一条直线上。
 *
 *
 * ## 示例 1：
 *
 * - 输入：points = [[1,1],[2,2],[3,3]]
 * - 输出：3
 *
 *
 * ## 示例 2：
 *
 * - 输入：points = [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]
 * - 输出：4
 *
 *
 * ## 提示：
 *
 * - points[i].length == 2
 * - -10^4 i, yi
 * - points 中的所有点 互不相同
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - hashmap
    /// 1. 最多只有n个点都在一条直线上,
    /// 2. 在一条直线上的点, 到其中一个点(x0,y0)的斜率相等.
    ///    即(xi-x0)/(yi-y0)相等;
    /// 3. 从一个点出发, 依次计算其他点与该点的斜率, 统计相同斜率最大的点数
    ///    此时需要用到hashmap;
    /// 4. 由于斜率为float类型, 为了保证严格相等, 用约分后的(dx, dy)来代替;
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let n = points.len();
        if n <= 2 {
            return n as i32;
        }

        // 辗转相除法计算a, b最大公约数
        fn gcd(a: i32, b: i32) -> i32 {
            assert!(a != 0 && b != 0);
            let (a, b) = (a.abs(), b.abs());
            let (mut a, mut b) = if a > b { (a, b) } else { (b, a) };
            while a % b != 0 {
                let c = a % b;
                if c > b {
                    a = c;
                } else {
                    a = b;
                    b = c;
                }
            }
            b
        }

        let mut max_count = 0;
        for i in 0..n - 1 {
            let mut lines = HashMap::new();
            let (x1, y1) = (points[i][0], points[i][1]);
            for j in i + 1..n {
                let (x2, y2) = (points[j][0], points[j][1]);
                let (dx, dy) = match (x1 - x2, y1 - y2) {
                    (0, _) => (0, 1),
                    (_, 0) => (1, 0),
                    _ => {
                        let (dx, dy) = (x1 - x2, y1 - y2);
                        let sig = if dx * dy > 0 { 1 } else { -1 };
                        let gcd = gcd(dx.abs(), dy.abs());
                        (sig * dx.abs() / gcd, dy.abs() / gcd)
                    }
                };
                let count = lines.entry((dx, dy)).or_insert(1);
                *count += 1;
                max_count = max_count.max(*count);
            }
        }

        max_count
    }
}
// @lc code=end
