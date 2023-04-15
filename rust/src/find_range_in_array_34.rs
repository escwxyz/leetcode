use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    /// Find range of target number in sorted array
    ///
    /// * `nums`: array of number
    /// * `target`: target number
    ///
    /// Problem: https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array
    ///
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() || !nums.contains(&target) {
            return vec![-1, -1];
        }

        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        let mut start = left;
        let mut end = left;

        if start > 0 && nums[start - 1] == target {
            while start > 0 && nums[start - 1] == target {
                start -= 1;
            }
        }

        if end < nums.len() - 1 && nums[end + 1] == target {
            while end < nums.len() - 1 && nums[end + 1] == target {
                end += 1;
            }
        }

        vec![start as i32, end as i32]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(
            vec![3, 4],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8)
        );
    }
    #[test]
    fn test_two() {
        assert_eq!(
            vec![-1, -1],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6)
        );
    }
    #[test]
    fn test_three() {
        assert_eq!(vec![-1, -1], Solution::search_range(vec![], 0));
    }

    #[test]
    fn test_four() {
        assert_eq!(
            vec![2, 5],
            Solution::search_range(vec![1, 2, 3, 3, 3, 3, 4, 5, 9], 3)
        )
    }
}
