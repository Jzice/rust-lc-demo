//! # 排序算法
//!
//! ## 总结
//!
//! |  分类  | 排序方法 | 时间复杂度(Avg/Max/Min)                 | 空间复杂度  | 稳定性 |
//! | ------ | -------- | --------------------------------------- | ----------- | ------ |
//! |  比较  | 冒泡排序 | O(n^2) / O(n^2) / O(n)                  | O(1)        | 稳定   |
//! |        | 插入排序 | O(n^2) / O(n^2) / O(n)                  | O(1)        | 稳定   |
//! |        | 选择排序 | O(n^2) / O(n^2) / O(n^2)                | O(1)        | 不稳定 |
//! |        | 希尔排序 | O(n^{1.3}) / O(n^2) / O(n)              | O(1)        | 不稳定 |
//! |        | 归并排序 | O(n*log(n)) / O(n*log(n)) / O(n*log(n)) | O(n)        | 稳定   |
//! |        | 堆排序   | O(n*log(n)) / O(n*log(n)) / O(n*log(n)) | O(1)        | 不稳定 |
//! |        | 快速排序 | O(n*log(n))/O(n^2)/O(n*log(n))          | O(n*log(n)) | 不稳定 |
//! | 非比较 | 基数排序 | O(n*k) / O(n*k) / O(n*k)                | O(n+k)      | 稳定   |
//! |        | 计数排序 | O(n+k) / O(n+k) / O(n+k)                | O(n+k)      | 稳定   |
//! |        | 桶排序   | O(n+k) / O(n^2) / O(n)                  | O(n+k)      | 稳定   |
//!

/// # 冒泡排序
/// ## 基本思想
/// 1. 遍历nums[], 依次比较相邻的两个元素nums[i], nums[i+1],
///    如果nums[i] > nums[i+1], 则交换两者位置;
/// 2. 遍历1次后, nums[0..n]中的最大的元素将位于数组尾部;
/// 3. 逐步减少遍历长度, 重复执行上述操作, 则每次将剩余数组中的最大元素交换到后面,
///    最终得到完整的有序数组;
pub fn bubble_sort<T>(nums: &mut Vec<T>)
    where T: PartialOrd
{
    for i in 1..nums.len() {        // 外层控制遍历的次数
        let mut sorted = true;
        for j in 0..nums.len()-i {  // 内存控制每次遍历比较的次数
            if nums[j] > nums[j+1] {
                nums.swap(j, j+1);
                sorted = false;
            }
        }
        if sorted {
            break;
        }
    }
}

/// # 鸡尾酒排序
/// ## 基本思想
/// 1. 从左右两个方向同时执行冒泡; 
/// 2. 遍历一次, 同时将最大最小值移动到数组两端;
pub fn cocktail_shaker_sort<T>(nums: &mut Vec<T>)
    where T: PartialOrd
{
    let L = nums.len();
    let mut i = 0;
    while i + 1 < L - i {
        let mut sorted = true;
        for k in i..(L-1-i) {
            if  nums[k] > nums[k+1] {
                nums.swap(k, k+1);
                sorted = false;
            }
            if  nums[L-1-k] < nums[L-1-k-1] {
                nums.swap(L-1-k, L-1-k-1);
                sorted = false;
            }
        }
        if sorted {
            break;
        }
        i += 1;
    }
}

/// # 插入排序
/// ## 基本思想
/// 1. 将待排序数组分为前后两个部分: 
///    - 前面: `已排序部分`;
///    - 后面: `未排序部分`;
/// 2. 开始`已排序部分`长度为1, `未排序部分`长度为`nums.len()-1`;
/// 3. 将`未排序部分`的头元素**插入**到前面`已排序部分`的正确位置(按大小顺序).
///    插入时，要将`已排序部分`插入点后面的元素依次往后移1位;
/// 4. 重复步骤3, 当所有`未排序部分`都插入到`已排序部分`后, 排序完成;
pub fn insert_sort<T>(nums: &mut Vec<T>)
    where T: PartialOrd + Copy + Clone
{
    for i in 1..nums.len() {
        let mut j = i;
        let n = nums[i];               // `未排序部分`头元素
        while j > 0 && nums[j-1] > n { // 从后往前将`已排序部分`中大于未排序部分头元素的元素
            nums[j] = nums[j-1];       // 后移动一位
            j -= 1;
        }
        nums[j] = n;                   // 将头元素插入到正确位置
    }
}

/// # 选择排序
/// ## 基本思想
/// 1. 将待排序数组分为前后两个部分: 
///    - 前面: `已排序部分`;
///    - 后面: `未排序部分`;
/// 2. 起始时, `已排序部分`长度为0, `未排序部分`长度为`nums.len()`;
/// 3. 每次从后面`未排序部分`中**选择**最小的元素, 将其放到前面`已排序部分`的末尾.
/// 4. 重复步骤3, 当所有`未排序部分`都选完, 则所有元素将按序加入到`已排序部分`, 算法结束;
pub fn select_sort<T>(nums: &mut Vec<T>)
    where T: PartialOrd
{
    for i in 0..nums.len() {                 
        let mut m = i;                  // 未排序部分最小元素初始下标
        for j in i+1..nums.len() {      // 遍历未排序部分, 寻找最小元素下标
            if nums[j] < nums[m] {      //
                m = j;
            }
        }
        nums.swap(i, m);                // 将最小元素和未排序部分最前面元素交换,
                                        // 即紧接已排序部分之后
    }
}

/// # 归并排序
/// ## 基本思想
/// 1. 自底向上, 依次将两个已排序好的子数组合并为一个大的已排序数组 
pub fn merge_sort<T>(nums: &mut Vec<T>) 
    where T: PartialOrd + Copy + Clone
{
    /// 将数组左右两个有序的部分合并, 使整体有序
    fn _merge<T>(nums: &mut Vec<T>, start: usize, mid: usize, end: usize)
         where T: PartialOrd + Copy + Clone
    {
        let nums1 = nums[start..mid].iter().cloned().collect::<Vec<_>>();
        let nums2 = nums[mid..end].iter().cloned().collect::<Vec<_>>();
        let (mut l, mut r) = (0, 0);
        while l < nums1.len() || r < nums2.len() {
            if r == nums2.len() || (l < nums1.len() && nums1[l] < nums2[r]) {
                nums[start+l+r] = nums1[l];
                l += 1;
            } else {
                nums[start+l+r] = nums2[r];
                r += 1;
            }
        }
    }

    /// 递归归并排序
    fn _merge_sort<T>(nums: &mut Vec<T>, start: usize, end: usize)
         where T: PartialOrd + Copy + Clone
    {
        if start + 1 >= end {
            return;
        }
        let m = (start + end ) / 2;
        _merge_sort(nums, start, m);
        _merge_sort(nums, m, end);
        _merge(nums, start, m, end);
    }

    _merge_sort(nums, 0, nums.len())
}

/// # 希尔排序
/// ## 基本思想
/// 1. 分组插入
pub fn shell_sort<T>(nums: &mut Vec<T>)
    where T: PartialOrd + Copy
{
    // 带间隔的插入排序
    fn _insert_sort_with_gap<T: PartialOrd + Copy>(nums: &mut Vec<T>, start: usize, gap: usize) {
        for i in (start+gap..nums.len()).step_by(gap) {
            let mut j = i;
            let n = nums[i];               // `未排序部分`头元素
            while j >= gap && nums[j-gap] > n { // 从后往前将`已排序部分`中大于未排序部分头元素的元素
                nums[j] = nums[j-gap];       // 后移动gap位
                j -= gap;
            }
            nums[j] = n;                   // 将头元素插入到正确位置
        }
    }

    let mut gap = nums.len() / 2;
    while gap > 0 {
        for start in 0..gap {
            _insert_sort_with_gap(nums, start, gap);
        }
        gap /= 2;
    }
}

/// # 堆排序
/// ## 基本思想
/// 1. 利用堆的特性对数组元素进行排序;
/// 2. 先将数组建堆(大顶堆);
/// 3. 然后依次取出堆顶元素(未排序部分的最大元素), 放置到数组末尾;
/// 4. 继续调整堆, 重复3操作, 直到所有元素都取完, 最终完成排序;
pub fn heap_sort<T>(nums: &mut Vec<T>)
    where T: PartialOrd
{
    // 堆化, 将nums[top..]调整为堆(大顶堆)
    fn _heapify<T: PartialOrd>(nums: &mut [T], top: usize) {
        let mut m = top;
        let (l, r) = ((2 * top + 1), (2 * top + 2));
        if l < nums.len() && nums[l] > nums[m] {
            m = l;
        }
        if r < nums.len() && nums[r] > nums[m] {
            m = r;
        }
        if m != top {
            nums.swap(m, top);
            _heapify(nums, m);
        }
    }

    // 从最后一个非叶子节点开始, 往上依次调整所有以该节点为根的子树，使之成为大顶堆
    for i in (0..nums.len()/2).rev() {
        _heapify(&mut nums[..], i);
    }

    // 依次取出堆顶元素, 和尾部元素交换
    for i in (0..nums.len()).rev() {
        nums.swap(0, i);
        _heapify(&mut nums[..i], 0)
    }
}

/// # 快速排序
/// ## 基本思想
/// 1. 在待排序序列中选定一个基准元素pivot；
/// 2. 以基准元素为标准, 将待排序数组划分为前后两个部分, 使得:
///    - 前一部分的每个元素都< 基准元素,
///    - 后一部分的每个元素都> 基准元素;
/// 3. 分别对两个部分重复上述操作, 直到长度<2;
pub fn quick_sort<T>(nums: &mut Vec<T>)
    where T: PartialOrd
{
    /// 将nums划分为前后两个部分, 使前一部分都小于后一部分, 返回划分点
    fn _partition<T: PartialOrd>(nums: &mut [T]) -> usize {
        let mut p = 0;              // 划分点
        let vi = nums.len() - 1;     // 选定尾部元素作为基准元素
        for i in 0..vi {             // 依次遍历所有非基准元素
            if nums[i] <= nums[vi] { // 如果当前元素<基准元素
                nums.swap(i, p);    // 将其和划分点元素交换
                p += 1;             // 后移划分点
            }
        }
        nums.swap(p, vi);            // 最后, 将基准元素和划分点元素交换
        p                           // 返回划分点
    }

    /// 递归快排
    fn _quick_sort<T: PartialOrd>(nums: &mut [T]) {
        if nums.len() < 2 {
            return;
        }
        let m = _partition(nums);        // 将nums划分为前后两部分
        _quick_sort(&mut nums[..m]);    // 
        _quick_sort(&mut nums[m+1..]);  //
    }

    _quick_sort(&mut nums[..]);
}

/// # 计数排序
/// ## 基本思路
/// - 空间换时间
/// 1. 计算nums中的最小值mi, 最大值ma;
/// 2. 按(ma-mi+1)大小申请count[]计数数组;
/// 3. 遍历nums, 统计其中每个元素n出现的次数, 将其计入n-ma为下标的count中;
/// 4. 遍历count, 计算每个下标累加的出现的次数;
/// 5. 复制一份nums副本;
/// 6. 遍历nums副本, 将各个元素n按照count中相对于mi的累加次数重新放置到原数组nums中;
pub fn count_sort(nums: &mut Vec<i32>) {
    if nums.len() == 0 {
        return;
    }
    let mi = *nums.iter().min().unwrap();
    let ma = *nums.iter().max().unwrap();
    let mut count = vec![0; (ma - mi + 1) as usize];
    // 统计每个数字出现的次数
    for n in nums.clone() {
        count[(n-mi) as usize] += 1;
    }
    // 统计每个数字出现的次数累加值
    for i in 1..count.len() {
        count[i] += count[i-1];
    }
    // 按累加值重新排序
    for n in nums.clone() {
        nums[count[(n - mi) as usize]-1] = n;
        count[(n-mi) as usize] -= 1;
    }
}

/// # 桶排序
/// ## 基本思想
/// - 计数排序的变种
/// 1. 桶是一个二级数组；
/// 2. 通过一个映射函数, 先将待排序序列中的各个元素映射到不同的桶中
/// 3. 然后再分别对桶中的元素进行排序;
/// 4. 最后依次将各个桶中的元素合并, 得到完整有序的序列;
pub fn bucket_sort(nums: &mut Vec<i32>) {
    if nums.len() == 0 {
        return;
    }
    let l = nums.len();
    let (mi, ma) = (*nums.iter().min().unwrap(), *nums.iter().max().unwrap());  
    if mi == ma {  
        // 所有元素大小相等, 则不用排序
        return;  
    }
    // 分配桶空间
    let mut buckets = vec![Vec::<i32>::new(); l+1];
    // 分桶, 将nums各个元素分到相应的桶中
    for &n in nums.iter() {
        let i = ((n - mi) as usize) * l / ((ma - mi) as usize);
        buckets[i].push(n);
    }
    // 将各桶中元素排序, 然后按顺序放回原数组
    let mut i = 0;
    for mut b in buckets {
        b.sort();
        for n in b {
            nums[i] = n;
            i += 1;
        }
    }
}


#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;
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

    fn sort_fn_bench(sort_fn: SortFn) {
        let mut nums1 = vec![10, 3, 26, 8, 7, 31, -20, 55, 68, 128, 
                            38, 101, -17, 9, 23, 84, 39, 205, 47, 27,
                            83, 171, -51, 49, 35, 44, 90, 235, 73, 22,
                        ];
        sort_fn(&mut nums1);
    }

    #[test]
    fn bubble_sort_test() {
        sort_fn_test(bubble_sort);
    }

    #[test]
    fn cocktail_shaker_sort_test() {
        sort_fn_test(cocktail_shaker_sort);
    }

    #[test]
    fn insert_sort_test() {
        sort_fn_test(insert_sort);
    }

    #[test]
    fn select_sort_test() {
        sort_fn_test(select_sort);
    }

    #[test]
    fn merge_sort_test() {
        sort_fn_test(merge_sort);
    }

    #[test]
    fn shell_sort_test() {
        sort_fn_test(shell_sort);
    }

    #[test]
    fn heap_sort_test() {
        sort_fn_test(heap_sort);
    }

    #[test]
    fn quick_sort_test() {
        sort_fn_test(quick_sort);
    }

    #[test]
    fn count_sort_test() {
        sort_fn_test(count_sort);
    }

    #[test]
    fn bucket_sort_test() {
        sort_fn_test(bucket_sort);
    }

    #[bench]
    fn bubble_sort_bench(b: &mut Bencher) {
        b.iter(|| {
            sort_fn_bench(bubble_sort);
        });
    }

    #[bench]
    fn cocktail_shaker_sort_bench(b: &mut Bencher) {
        b.iter(|| {
            sort_fn_bench(cocktail_shaker_sort);
        });
    }

    #[bench]
    fn select_sort_bench(b: &mut Bencher) {
        b.iter(|| {
            sort_fn_bench(select_sort);
        });
    }

    #[bench]
    fn insert_sort_bench(b: &mut Bencher) {
        b.iter(|| {
            sort_fn_bench(insert_sort);
        });
    }

    #[bench]
    fn merge_sort_bench(b: &mut Bencher) {
        b.iter(|| {
            sort_fn_bench(merge_sort);
        });
    }

    #[bench]
    fn shell_sort_bench(b: &mut Bencher) {
        b.iter(|| {
            sort_fn_bench(shell_sort);
        });
    }

    #[bench]
    fn count_sort_bench(b: &mut Bencher) {
        b.iter(|| {
            sort_fn_bench(count_sort);
        });
    }

    #[bench]
    fn bucket_sort_bench(b: &mut Bencher) {
        b.iter(|| {
            sort_fn_bench(bucket_sort);
        });
    }

    #[bench]
    fn heap_sort_bench(b: &mut Bencher) {
        b.iter(|| {
            sort_fn_bench(heap_sort);
        });
    }

    #[bench]
    fn quick_sort_bench(b: &mut Bencher) {
        b.iter(|| {
             sort_fn_bench(quick_sort);
        });
    }
}
