/// alg/sort.rs
/// 排序算法

//

/// # 冒泡排序
/// ## 基本思想
/// 1. 遍历nums, 依次比较相邻的两个元素nums[i], nums[i+1],
///    如果nums[i] > nums[i+1], 则交换两者位置, 
/// 2. 遍历完成后最大的元素将位于数组尾部;
/// 3. 逐步减少nums长度, 执行上述操作, 每次将剩余元素中的最大元素交换到后面,
///    最终得到完整的有序数组;
pub fn bubble_sort<T>(nums: &mut Vec<T>)
    where T: PartialOrd
{
    if nums.len() < 2 {
        return;
    }
    for i in 0..nums.len()-1 {
        for j in 0..nums.len()-1-i {
            if nums[j] > nums[j+1] {
                nums.swap(j, j+1);
            }
        }
    }
}

/// # 插入排序
/// ## 基本思想
/// 1. 将待排序数组分为前后两个部分: 
///    - 前面已排序部分;
///    - 后面未排序部分;
/// 2. 每次选择后面未排序部分一个元素, 将其插入到前面已排序部分的正确位置
///    插入中间位置时，要将后面有序元素依次往后移1位;
/// 3. 当所有未排序部分都插入到前面部分后, 排序完成;
pub fn insert_sort<T>(nums: &mut Vec<T>)
    where T: PartialOrd
{
    if nums.len() < 2 {
        return;
    }
    for i in 0..nums.len() - 1 {
        for j in i+1..nums.len() {
            todo!()
        }
    }
    todo!()
}

/// # 选择排序
/// ## 基本思路
/// 1. 遍历一遍nums, 通过比较可以找到最小元素;
/// 2. 因此, 每次遍历将nums中剩下的最小元素取出, 放到nums前面已排序的末尾, 
///    则可依次将所有元素排序;
pub fn select_sort<T>(nums: &mut Vec<T>)
    where T: PartialOrd
{
    let l = nums.len();
    for i in 0..l {                 // nums[i]表示已选择的第i小元素
        for j in i+1..l {           // nums[j]表示nums[i..]后的元素
            // 如果当前元素 比 之前已遍历的元素更小 
            if nums[j] < nums[i] {
                nums.swap(i, j);    // 将其交换到选定已排序好的
            }
        }
    }
}


/// # 归并排序
/// ## 基本思想
/// 1. 
pub fn merge_sort<T>(nums: &mut Vec<T>) 
    where T: PartialOrd
{
    todo!()
}

/// # 希尔排序
/// ## 基本思想
/// 1. 
pub fn shell_sort<T>(nums: &mut Vec<T>)
    where T: PartialOrd
{
    todo!()
}

/// # 堆排序
/// ## 基本思想
pub fn heap_sort<T>(nums: &mut Vec<T>)
    where T: PartialOrd
{
    // 以nums[top]为顶调整为堆
    fn heapify<T: PartialOrd>(nums: &mut [T], top: usize) {
        let mut m = top;
        let (l, r) = (2 * top, 2 * top + 1);
        if l < nums.len() && nums[l] >= nums[m] {
            m = l;
        }
        if r < nums.len() && nums[r] >= nums[m] {
            m = r;
        }
        if m != top {
            nums.swap(m, top);
            heapify(nums, m);
        }
    }

    // 从最后一个非叶子节点开始, 往上依次调整所有以该节点为根的子树，使之成为大顶堆
    for i in (0..nums.len()/2).rev() {
        heapify(&mut nums[..], i);
    }

    // 依次取出堆顶元素, 和尾部元素交换
    for i in (0..nums.len()).rev() {
        nums.swap(0, i);
        heapify(&mut nums[..i], 0)
    }
}

/// # 快速排序
/// ## 基本思想
/// 1. 在待排序数组中选定一个基准元素pivot；
/// 2. 以基准元素为标准, 将待排序数组划分为前后两个部分,
///    使前一部分的任意元素都< 基准元素,
///    而后一部分的任意元素都> 基准元素;
/// 3. 分别对两个部分重复上述操作, 直到数组长度<2;
pub fn quick_sort<T>(nums: &mut Vec<T>)
    where T: PartialOrd
{
    // 将nums划分为前后两个部分, 使前一部分都小于后一部分, 返回划分点
    fn partition<T: PartialOrd>(nums: &mut [T]) -> usize {
        let mut m = 0;              // 划分点
        let p = nums.len() - 1;     // 基准元素下标
        for i in 0..p {             // 依次遍历所有非基准元素
            if nums[i] <= nums[p] { // 如果当前元素<基准元素
                nums.swap(i, m);    // 将其和划分点元素交换
                m += 1;             // 往后移动划分点
            }
        }
        nums.swap(m, p);            // 最后, 将基准元素和划分点元素交换
        m                           // 返回划分点
    }

    // 递归快排
    fn _quick_sort<T: PartialOrd>(nums: &mut [T]) {
        if nums.len() < 2 {
            return;
        }
        let m = partition(nums);        // 将nums划分为前后两部分
        _quick_sort(&mut nums[..m]);    // 
        _quick_sort(&mut nums[m+1..]);  //
    }

    _quick_sort(&mut nums[..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    type SortFn = fn(&mut Vec<i32>);

    fn sort_fn_test(sort_fn: SortFn) {
        let mut nums1 = vec![10, 3, 26, 8, 7, 31];
        sort_fn(&mut nums1);
        assert_eq!(nums1, vec![3, 7, 8, 10, 26, 31]);

        let mut nums1 = vec![10, 3, 26, 8, 7, 31, -1];
        sort_fn(&mut nums1);
        assert_eq!(nums1, vec![-1, 3, 7, 8, 10, 26, 31]);

        let mut nums1 = vec![10, 3, 7, 26, 8, 7, 31, -1];
        sort_fn(&mut nums1);
        assert_eq!(nums1, vec![-1, 3, 7, 7, 8, 10, 26, 31]);

        let mut nums1 = vec![10, 3, 7, 26, 8, 7, 31, -1, 26];
        sort_fn(&mut nums1);
        assert_eq!(nums1, vec![-1, 3, 7, 7, 8, 10, 26, 26, 31]);

        let mut nums1 = vec![];
        sort_fn(&mut nums1);
        assert_eq!(nums1, vec![]);

        let mut nums1 = vec![1];
        sort_fn(&mut nums1);
        assert_eq!(nums1, vec![1]);

        let mut nums1 = vec![2, 1];
        sort_fn(&mut nums1);
        assert_eq!(nums1, vec![1, 2]);
    }

    #[test]
    fn bubble_sort_test() {
        sort_fn_test(bubble_sort);
    }

    #[test]
    fn heap_sort_test() {
        sort_fn_test(heap_sort);
    }

    #[test]
    fn quick_sort_test() {
        sort_fn_test(quick_sort);
    }
}
