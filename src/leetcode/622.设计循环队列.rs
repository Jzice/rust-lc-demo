/*!
 * # [622.设计循环队列]( https://leetcode.cn/problems/design-circular-queue/description/)
 *
 * @lc app=leetcode.cn id=622 lang=rust
 *
 * ## 难度
 *
 * - Medium (43.77%)
 * - Likes:    227
 * - Dislikes: 0
 * - Total Accepted:    67.8K
 * - Total Submissions: 154.8K
 * - Testcase Example:  '["MyCircularQueue","enQueue","enQueue","enQueue","enQueue","Rear","isFull","deQueue","enQueue","Rear"]\n' +
 * '[[3],[1],[2],[3],[4],[],[],[],[4],[]]'
 *
 * ## 问题描述
 *
 * 设计你的循环队列实现。 循环队列是一种线性数据结构，其操作表现基于
 *
 * FIFO（先进先出）原则并且队尾被连接在队首之后以形成一个循环。它也被称为“环形缓冲器”。
 * 
 * 循环队列的一个好处是我们可以利用这个队列之前用过的空间。在一个普通队列里，一旦一个队列满了，我们就不能插入下一个元素，即使在队列前面仍有空间。但是使用循环队列，我们能使用这些空间去存储新的值。
 * 
 * 你的实现应该支持如下操作：
 * 
 * ```text
 * MyCircularQueue(k): 构造器，设置队列长度为 k 。
 * Front: 从队首获取元素。如果队列为空，返回 -1 。
 * Rear: 获取队尾元素。如果队列为空，返回 -1 。
 * enQueue(value): 向循环队列插入一个元素。如果成功插入则返回真。
 * deQueue(): 从循环队列中删除一个元素。如果成功删除则返回真。
 * isEmpty(): 检查循环队列是否为空。
 * isFull(): 检查循环队列是否已满。
 * ```
 * 
 * ## 示例：
 * 
 * ```cpp
 * MyCircularQueue circularQueue = new MyCircularQueue(3); // 设置长度为 3
 * circularQueue.enQueue(1);  // 返回 true
 * circularQueue.enQueue(2);  // 返回 true
 * circularQueue.enQueue(3);  // 返回 true
 * circularQueue.enQueue(4);  // 返回 false，队列已满
 * circularQueue.Rear();  // 返回 3
 * circularQueue.isFull();  // 返回 true
 * circularQueue.deQueue();  // 返回 true
 * circularQueue.enQueue(4);  // 返回 true
 * circularQueue.Rear();  // 返回 4
 * ```
 * 
 * ## 提示：
 * - 所有的值都在 0 至 1000 的范围内；
 * - 操作数将在 1 至 1000 的范围内；
 * - 请不要使用内置的队列库。
 * 
 */

// @lc code=start

struct MyCircularQueue {
    data: Vec<i32>,
    head: i32,
    tail: i32,
    size: i32,
    capacity: i32,
}

impl MyCircularQueue {
    /// 构造器，设置队列长度为 k
    fn new(k: i32) -> Self {
        MyCircularQueue {
            data: vec![-1; k as usize],
            head: -1,
            tail: -1,
            size: 0,
            capacity: k,
        }
    }
    
    /// 向循环队列插入一个元素。如果成功插入则返回真
    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        if self.is_empty() {
            self.head = 0;
        }
        self.tail = (self.tail + 1) % self.capacity;
        self.data[self.tail as usize] = value;
        self.size += 1;
        true
    }
    
    /// 从循环队列中删除一个元素。如果成功删除则返回真
    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        if self.head == self.tail {
            self.head = -1;
            self.tail = -1;
        } else {
            self.head = (self.head + 1) % self.capacity;
        }
        self.size -= 1;
        true
    }
    
    /// 从队首获取元素。如果队列为空，返回 -1
    fn front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[self.head as usize]
    }
    
    /// 获取队尾元素。如果队列为空，返回 -1
    fn rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[self.tail as usize]
    }
    
    /// 检查循环队列是否为空
    fn is_empty(&self) -> bool {
        self.size == 0
    }
    
    /// 检查循环队列是否已满
    fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */
// @lc code=end

// 增加测试用例
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_queue() {
        let mut circular_queue = MyCircularQueue::new(3); // 设置长度为 3
        assert_eq!(circular_queue.en_queue(1), true);  // 返回 true
        assert_eq!(circular_queue.en_queue(2), true);  // 返回 true
        assert_eq!(circular_queue.en_queue(3), true);  // 返回 true
        assert_eq!(circular_queue.en_queue(4), false); // 返回 false，队列已满
        assert_eq!(circular_queue.rear(), 3);          // 返回 3
        assert_eq!(circular_queue.is_full(), true);    // 返回 true
        assert_eq!(circular_queue.de_queue(), true);   // 返回 true
        assert_eq!(circular_queue.en_queue(4), true);  // 返回 true
        assert_eq!(circular_queue.rear(), 4);          // 返回 4
    }
}

