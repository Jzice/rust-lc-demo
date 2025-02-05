/*!
 * # [299.猜数字游戏](https://leetcode.cn/problems/bulls-and-cows/description/)
 * 
 * @lc app=leetcode.cn id=299 lang=rust
 *
 * ## 难度
 * algorithms
 * Medium (57.61%)
 * Likes:    310
 * Dislikes: 0
 * Total Accepted:    87.6K
 * Total Submissions: 152K
 * Testcase Example:  '"1807"\n"7810"'
 *
 * ## 描述
 * 
 * 你在和朋友一起玩 猜数字（Bulls and Cows）游戏，该游戏规则如下：
 * 
 * 写出一个秘密数字，并请朋友猜这个数字是多少。朋友每猜测一次，你就会给他一个包含下述信息的提示：
 * 
 * 
 * 猜测数字中有多少位属于数字和确切位置都猜对了（称为 "Bulls"，公牛），
 * 有多少位属于数字猜对了但是位置不对（称为 "Cows"，奶牛）。也就是说，这次猜测中有多少位非公牛数字可以通过重新排列转换成公牛数字。
 * 
 * 
 * 给你一个秘密数字 secret 和朋友猜测的数字 guess ，请你返回对朋友这次猜测的提示。
 * 
 * 提示的格式为 "xAyB" ，x 是公牛个数， y 是奶牛个数，A 表示公牛，B 表示奶牛。
 * 
 * 请注意秘密数字和朋友猜测的数字都可能含有重复数字。
 * 
 * 
 * 
 * ## 示例 1：
 * 
 * 输入：secret = "1807", guess = "7810"
 * 输出："1A3B"
 * 解释：数字和位置都对（公牛）用 '|' 连接，数字猜对位置不对（奶牛）的采用斜体加粗标识。
 * "1807"
 * ⁠ |
 * "7810"
 * 
 * ## 示例 2：
 * - 输入：secret = "1123", guess = "0111"
 * - 输出："1A1B"
 * - 解释：数字和位置都对（公牛）用 '|' 连接，数字猜对位置不对（奶牛）的采用斜体加粗标识。
 * "1123"        "1123"
 * ⁠ |      or     |
 * "0111"        "0111"
 * 注意，两个不匹配的 1 中，只有一个会算作奶牛（数字猜对位置不对）。通过重新排列非公牛数字，其中仅有一个 1 可以成为公牛数字。
 * 
 * 
 * 
 * ## 提示：
 * - 1 <= secret.length, guess.length <= 1000
 * - secret.length == guess.length
 * - secret 和 guess 仅由数字组成
 * 
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    /// # 猜数字游戏
    pub fn get_hint(secret: String, guess: String) -> String {
        use std::collections::HashMap;

        assert!(secret.len() == guess.len());

        let mut char_cnt = secret.bytes()
            .fold(HashMap::new(), |mut acc, c|{
            acc.entry(c)
                .and_modify(|e|*e += 1)
                .or_insert(1);
            acc
        });

        let (x, y) = secret.bytes()
            .zip(guess.bytes())
            .fold((0, 0), |(mut x, mut y), c|{
                if let Some(_) = char_cnt.get(&c.1)
                    .filter(|&cnt| *cnt > 0) {
                        char_cnt.entry(c.1).and_modify(|e| *e-=1);
                        if c.0 == c.1 {
                            x += 1;
                        } else {
                            y += 1;
                        }
                } 
                (x, y)
            });

        format!("{}A{}B", x, y)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::get_hint(
            "1807".into(), 
            "7810".into()
            ), 
            "1A3B".to_string()
        );

        assert_eq!(Solution::get_hint(
            "1123".into(), 
            "0111".into()), 
            "1A1B".to_string()
        );
    }
}

