use std::collections::HashMap;

use super::Solution;

impl Solution {
    /// 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
    ///
    /// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
    ///
    /// 你可以按任意顺序返回答案。
    ///
    /// 提示：
    ///
    /// 2 <= nums.length <= 104  
    /// -109 <= `nums[i]` <= 109  
    /// -109 <= target <= 109  
    /// 只会存在一个有效答案  
    ///
    /// 进阶：你可以想出一个时间复杂度小于 O(n2) 的算法吗？
    ///
    /// 来源：力扣（LeetCode）  
    /// 链接：<https://leetcode.cn/problems/two-sum>  
    /// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。  
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());

        for (i, num) in nums.into_iter().enumerate() {
            match map.get(&(target - num)) {
                None => map.insert(num, i),
                Some(j) => return vec![i as i32, *j as i32],
            };
        }

        // println!("{:#?}", map);

        panic!("题目说了有解");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(result: Vec<i32>, mut answer: Vec<i32>) {
        if result == answer {
            return;
        }
        answer.reverse();
        assert_eq!(result, answer);
    }

    #[test]
    fn examples() {
        check(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        check(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        check(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
