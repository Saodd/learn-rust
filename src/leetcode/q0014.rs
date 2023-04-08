use std::str::Chars;

use super::Solution;

impl Solution {
    /// 14. 最长公共前缀（简单）
    ///
    /// 编写一个函数来查找字符串数组中的最长公共前缀。
    ///
    /// 如果不存在公共前缀，返回空字符串 ""。
    ///
    /// 提示：
    ///
    /// 1 <= `strs.length` <= 200  
    /// 0 <= `strs[i].length` <= 200  
    /// `strs[i]` 仅由小写英文字母组成  
    ///
    /// 来源：力扣（LeetCode）  
    /// 链接：<https://leetcode.cn/problems/longest-common-prefix/>  
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut res = String::with_capacity(strs[0].len());

        let mut its = strs.iter().map(|s| s.chars());
        let it0: Chars = its.next().expect("1 <= strs.length");
        let mut its: Vec<Chars> = its.collect();

        for char0 in it0 {
            for it in its.iter_mut() {
                match it.next() {
                    None => return res,
                    Some(char1) => {
                        if char0 != char1 {
                            return res;
                        }
                    }
                }
            }
            res.push(char0);
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prepare(strs: Vec<&str>) -> Vec<String> {
        let mut res: Vec<String> = Vec::with_capacity(strs.len());
        for s in strs {
            res.push(s.to_string())
        }
        return res;
    }

    #[test]
    fn examples() {
        assert_eq!(
            Solution::longest_common_prefix(prepare(vec!["flower", "flow", "flight"])),
            "fl".to_string()
        );
        assert_eq!(
            Solution::longest_common_prefix(prepare(vec!["dog", "racecar", "car"])),
            "".to_string()
        );
    }

    // #[test]
    // fn check_myself() {}
    //
    // #[test]
    // fn commit_failed() {}
}
