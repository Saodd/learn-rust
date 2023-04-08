use super::Solution;

impl Solution {
    /// 7. 整数反转（中等）
    ///
    /// 给你一个 32 位的有符号整数 x ，返回将 x 中的数字部分反转后的结果。
    ///
    /// 如果反转后整数超过 32 位的有符号整数的范围 `[−2^31,  2^31 − 1]` ，就返回 0。
    ///
    /// 假设环境不允许存储 64 位整数（有符号或无符号）。
    ///
    /// 提示：
    ///
    /// -2^31 <= x <= 2^31 - 1
    ///
    /// 来源：力扣（LeetCode）  
    /// 链接：<https://leetcode.cn/problems/reverse-integer/>  
    pub fn reverse(x: i32) -> i32 {
        if x > 0 {
            return -Solution::reverse(-x); // 因为不可能得到 i32::MAX 的返回值，所以取负数是安全的
        };
        if x == 0 {
            return 0;
        };

        let mut num = x;
        let mut res = 0;

        while num != 0 {
            let digit = num % 10;
            num = num / 10;

            match i32::checked_mul(res, 10).and_then(|v| i32::checked_add(v, digit)) {
                None => return 0,
                Some(v) => res = v,
            }
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(0), 0);
    }

    #[test]
    fn check_myself() {
        assert_eq!(Solution::reverse(i32::MIN), 0); // -2,147,483,648
        assert_eq!(Solution::reverse(i32::MAX), 0); // 2,147,483,647
    }

    #[test]
    fn commit_failed() {
        // 用 res > 0 来判断整数越界并不靠谱，还是得老老实实用 i32::checked_* 函数
        assert_eq!(Solution::reverse(1_534_236_469), 0);
    }
}
