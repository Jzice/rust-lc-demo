/*!
 * # [210.课程表II](https://leetcode.cn/problems/course-schedule-ii/description/)
 *
 * @lc app=leetcode.cn id=210 lang=rust
 *
 * ## 难度
 * - Medium (56.67%)
 * - Likes:    800
 * - Dislikes: 0
 * - Total Accepted:    185K
 * - Total Submissions: 326.3K
 * - Testcase Example:  '2\n[[1,0]]'
 *
 * ## 问题描述
 *
 * 现在你总共有 numCourses 门课需要选，记为 0 到 numCourses - 1。给你一个数组 prerequisites ，其中
 * prerequisites[i] = [ai, bi] ，表示在选修课程 ai 前 必须 先选修 bi 。
 *
 * 例如，想要学习课程 0 ，你需要先完成课程 1 ，我们用一个匹配来表示：[0,1] 。
 *
 * 返回你为了学完所有课程所安排的学习顺序。可能会有多个正确的顺序，你只要返回 任意一种 就可以了。如果不可能完成所有课程，返回 一个空数组 。
 *
 * ## 示例 1：
 * - 输入：numCourses = 2, prerequisites = [[1,0]]
 * - 输出：[0,1]
 * - 解释：总共有 2 门课程。要学习课程 1，你需要先完成课程 0。因此，正确的课程顺序为 [0,1] 。
 *
 * ## 示例 2：
 * - 输入：numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
 * - 输出：[0,2,1,3]
 * - 解释：总共有 4 门课程。要学习课程 3，你应该先完成课程 1 和课程 2。并且课程 1 和课程 2 都应该排在课程 0 之后。
 * 因此，一个正确的课程顺序是 [0,1,2,3] 。另一个正确的排序是 [0,2,1,3] 。
 *
 * ## 示例 3：
 * - 输入：numCourses = 1, prerequisites = []
 * - 输出：[0]
 *
 * ## 提示：
 * - 1 <= numCourses <= 2000
 * - 0 <= prerequisites.length <= numCourses * (numCourses - 1)
 * - prerequisites[i].length == 2
 * - 0 <= ai, bi < numCourses
 * - ai != bi
 * - 所有[ai, bi] 互不相同
 */
struct Solution;
// @lc code=start
impl Solution {
    /// # 课程表II
    /// ## 解题思路
    /// - bfs(同207.课程表)
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut in_degrees = vec![0; num_courses as usize];
        let mut g = vec![vec![]; num_courses as usize];
        for p in &prerequisites {
            let (to, from) = (p[0] as usize, p[1] as usize);
            in_degrees[to] += 1;
            g[from].push(to); //
        }

        let mut can_learn = in_degrees
            .iter()
            .enumerate()
            .filter(|(_, &d)| d == 0)
            .map(|(from, _)| from)
            .collect::<VecDeque<_>>();

        let mut res = vec![];
        let mut remain = num_courses as usize;
        while !can_learn.is_empty() {
            remain -= can_learn.len();
            res.append(
                &mut can_learn
                    .iter()
                    .clone()
                    .map(|&i| i as i32)
                    .collect::<Vec<_>>(),
            );
            for _ in 0..can_learn.len() {
                let from = can_learn.pop_front().unwrap();
                for &t in &g[from] {
                    in_degrees[t] -= 1;
                    if in_degrees[t] == 0 {
                        can_learn.push_back(t);
                    }
                }
            }
        }

        if remain == 0 {
            res
        } else {
            vec![]
        }

        //todo!()
    }
}
// @lc code=end
