/*
* @lc app=leetcode.cn id=2605 lang=rust slug=form-smallest-number-from-two-digit-arrays
*
* # 2605.从两个数字数组里生成最小数字
*
* https://leetcode.cn/problems/form-smallest-number-from-two-digit-arrays/description/
*
* Easy (69.26%)
*
给你两个只包含 1 到 9 之间数字的数组 nums1 和 nums2 ，每个数组中的元素 互不相同 ，请你返回 最小 的数字，两个数组都 至少 包含这个数字的某个数位。
 
示例 1：
输入：nums1 = [4,1,3], nums2 = [5,7]
输出：15
解释：数字 15 的数位 1 在 nums1 中出现，数位 5 在 nums2 中出现。15 是我们能得到的最小数字。
示例 2：
输入：nums1 = [3,5,2,6], nums2 = [3,1,7]
输出：3
解释：数字 3 的数位 3 在两个数组中都出现了。
 
提示：
	1 <= nums1.length, nums2.length <= 9
	1 <= nums1[i], nums2[i] <= 9
	每个数组中，元素 互不相同 。
*
* test case:
[4,1,3]
[5,7]
*/

use super::*;

// @lc code=start
impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let (mut nums1, mut nums2) = if nums1.len() < nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };
        nums2.sort();
        let nums1_set = nums1.iter().collect::<HashSet<_>>();
        for n2 in &nums2 {
            if nums1_set.contains(n2) {
                return  *n2;
            }
        }
        let n1 = nums1.iter().min().unwrap();
        let n2 = nums2.iter().min().unwrap();
        if *n1 > *n2 {
            if *n1 == 0 {
                return *n2;
            } else {
                return *n2 * 10 + *n1;
            }
        } else {
            if *n2 == 0 {
                return *n1;
            } else {
                return *n1* 10 + *n2;
            }
        }
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
    }
}
        
