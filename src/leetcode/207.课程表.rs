/*!
 * # [207.课程表](https://leetcode.cn/problems/course-schedule/description/)
 *
 * @lc app=leetcode.cn id=207 lang=rust
 *
 * ## 难度
 * - Medium (53.55%)
 * - Likes:    1660
 * - Dislikes: 0
 * - Total Accepted:    314.4K
 * - Total Submissions: 586.9K
 * - Testcase Example:  '2\n[[1,0]]'
 *
 * ## 问题描述
 *
 * 你这个学期必须选修 numCourses 门课程，记为 0 到 numCourses - 1 。
 *
 * 在选修某些课程之前需要一些先修课程。 先修课程按数组 prerequisites 给出，其中 prerequisites[i] = [ai, bi]
 * ，表示如果要学习课程 ai 则 必须 先学习课程  bi 。
 *
 * 例如，先修课程对 [0, 1] 表示：想要学习课程 0 ，你需要先完成课程 1 。
 *
 * 请你判断是否可能完成所有课程的学习？如果可以，返回 true ；否则，返回 false 。
 *
 *
 *
 * ## 示例 1：
 *
 * - 输入：numCourses = 2, prerequisites = [[1,0]]
 * - 输出：true
 * - 解释：总共有 2 门课程。学习课程 1 之前，你需要完成课程 0 。这是可能的。
 *
 * ## 示例 2：
 *
 * - 输入：numCourses = 2, prerequisites = [[1,0],[0,1]]
 * - 输出：false
 * - 解释：总共有 2 门课程。学习课程 1 之前，你需要先完成​课程 0 ；并且学习课程 0 之前，你还应先完成课程 1 。这是不可能的。
 *
 * ## 提示：
 *
 * - 1 <= numCourses <= 2000
 * - 0 <= prerequisites.length <= 5000
 * - prerequisites[i].length == 2
 * - 0 <= ai, bi < numCourses
 * - prerequisites[i] 中的所有课程对 互不相同
 *
 *
 */

use super::*;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - bfs
    /// 1. 遍历prerequisites, 记录每个课程的入度;
    /// 2. 收集所有入度为0的节点;
    /// 3. 从所有入度为0的节点开始, 逐级遍历邻接点;
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        use std::collections::VecDeque;

        let mut in_degrees = vec![0; num_courses as usize];
        let mut g = vec![vec![]; num_courses as usize];
        for p in &prerequisites {
            let (to, from) = (p[0] as usize, p[1] as usize);
            in_degrees[to] += 1; //入度
            g[from].push(to); // 节点单向邻接表
        }

        // 选择入度为0的叶子节点作为起始节点
        let mut can_learn = in_degrees
            .iter()
            .enumerate()
            .clone()
            .filter(|(_, &d)| d == 0)
            .map(|(i, _)| i)
            .collect::<VecDeque<_>>();

        let mut remain = num_courses as usize;
        // 从叶子节点开始, 逐层遍历各层节点
        while !can_learn.is_empty() {
            remain -= can_learn.len();
            // 否则如果存在可遍历的节点
            for _ in 0..can_learn.len() {
                let from = can_learn.pop_front().unwrap();
                for &to in &g[from] {
                    in_degrees[to] -= 1; // 将下级节点入度-1

                    // 如果下级节点的入度为0, 则为下次遍历的叶子节点
                    if in_degrees[to] == 0 {
                        can_learn.push_back(to); //收集入度为0的叶子节点
                    }
                }
            }
        }
        // 所有节点遍历完, 则课程修完, 返回true
        return remain == 0;
    }
}
// @lc code=end
