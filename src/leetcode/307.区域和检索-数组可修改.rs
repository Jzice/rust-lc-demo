/*!
 * # [307.区域和检索-数组可修改]( https://leetcode.cn/problems/range-sum-query-mutable/description/)
 *
 * @lc app=leetcode.cn id=307 lang=rust
 *
 * ## 难度
 * - Medium (52.17%)
 * - Likes:    619
 * - Dislikes: 0
 * - Total Accepted:    68.6K
 * - Total Submissions: 131.6K
 * - Testcase Example:  '["NumArray","sumRange","update","sumRange"]\n[[[1,3,5]],[0,2],[1,2],[0,2]]'
 *
 * ## 问题描述
 *
 * 给你一个数组 nums ，请你完成两类查询。
 * 
 * 其中一类查询要求 更新 数组 nums 下标对应的值
 * 另一类查询要求返回数组 nums 中索引 left 和索引 right 之间（ 包含 ）的nums元素的 和 ，其中 left <= right
 * 
 * 
 * 实现 NumArray 类：
 * 
 * ```cpp
 * NumArray(int[] nums); // 用整数数组 nums 初始化对象
 * void update(int index, int val); // 将 nums[index] 的值 更新 为 val
 * int sumRange(int left, int right); // 返回数组 nums 中索引 left 和索引 right 之间（ 包含 ）的nums元素的 和 （即，nums[left] + nums[left + 1], ..., nums[right]）
 * ``` 
 * 
 * ## 示例 1：
 * - 输入：
 * ["NumArray", "sumRange", "update", "sumRange"]
 * [[[1, 3, 5]], [0, 2], [1, 2], [0, 2]]
 * - 输出：[null, 9, null, 8]
 * - 解释：
 * ```cpp
 * NumArray numArray = new NumArray([1, 3, 5]);
 * numArray.sumRange(0, 2); // 返回 1 + 3 + 5 = 9
 * numArray.update(1, 2);   // nums = [1,2,5]
 * numArray.sumRange(0, 2); // 返回 1 + 2 + 5 = 8
 * ```
 * 
 * ## 提示：
 *
 * - 1 <= nums.length <= 3 * 10^4
 * - -100 <= nums[i] <= 100
 * - 0 <= index < nums.length
 * - -100 <= val <= 100
 * - 0 <= left <= right < nums.length
 * - 调用 update 和 sumRange 方法次数不大于 3 * 10^4 
 * 
 */

// @lc code=start
/// 树状数组
struct NumArray {
    lowbit_sums: Vec<i32>, //按lowbit方式存储的sum数组
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */
impl NumArray {
    /// 新建树状数组
    fn new(nums: Vec<i32>) -> Self {
        let mut na = NumArray {
            lowbit_sums: vec![0; nums.len()], // lowbit_sums数组
        };
        for i in 0..nums.len() {
            na.modify(i as i32, nums[i]);
        }

        na
    }

    /// 修改nums[i]值
    fn modify(&mut self, i: i32, diff: i32) {
        let mut i = i + 1; // 树状数组下标=原数组下标+1
        //自底向上修改树状数组相关节点
        while i <= self.lowbit_sums.len() as i32 {
            self.lowbit_sums[(i-1) as usize] += diff;
            i += NumArray::lowbit(i); 
        }
    }

    /// 查询前缀和sum(nums[..i]), 区间范围: [0, i)
    fn get_prefix_sum(&self, i: i32) -> i32 {
        let mut res = 0;
        let mut i = i + 1; // 树状数组下标=原数组下标+1
        //自顶向下依次计算树状数组节点和
        while i > 0 {
            res += self.lowbit_sums[(i-1) as usize];
            i -= NumArray::lowbit(i); 
        }

        res
    }

    /// 查询nums[i]
    fn get(&self, i: i32) -> i32 {
        self.get_prefix_sum(i) - self.get_prefix_sum(i-1)
    }

    /// 更新nums[index]值
    fn update(&mut self, i: i32, val: i32) {
        let diff = val - self.get(i);
        self.modify(i, diff)
    }
    
    /// 查询区间和sum(nums[left..right]), [left, right)
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        // 区间和转化为前缀和之差
        self.get_prefix_sum(right) - self.get_prefix_sum(left-1)
    }

    /// 截取x的最低位1开始后的尾部
    fn lowbit(x: i32) -> i32 {
        x & -x
    }
}

// @lc code=end
