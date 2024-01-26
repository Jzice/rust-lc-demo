// heap_sort.rs
//
//

type T = i32;

// 堆排序
pub fn heap_sort(nums: &mut Vec<T>)
{
    // 堆调整辅助函数
    fn heapify(nums: &mut [T], start: usize) {
        let mut top = start;
        let (l, r) = (2 * start, 2 * start + 1);
        if l < nums.len() && nums[l] > nums[top] {
            top = l;
        }
        if r < nums.len() && nums[r] > nums[top] {
            top = r;
        }
        if top != start {
            nums.swap(top, start);
            heapify(nums, top);
        }
    }

    // 从最后一个非叶子节点开始, 往上依次调整所有以该节点为根的子树，使之满足堆
    for i in (0..nums.len()/2).rev() {
        heapify(&mut nums[..], i);
    }

    // 循环取出堆顶元素, 和堆尾部
    for i in (0..nums.len()).rev() {
        nums.swap(0, i);
        heapify(&mut nums[..i], 0)
    }
}

pub fn quick_sort(nums: &mut Vec<T>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap_sort_test() {
        let mut nums = vec![10, 3, 26, 8, 7, 31];

        heap_sort(&mut nums);
        assert_eq!(nums, vec![3, 7, 8, 10, 26, 31]);

    }
}
