use crate::Solution;

impl Solution {
    #![allow(dead_code)]
    /// Problem: https://leetcode.com/problems/container-with-most-water/description/
    ///
    /// * `height`: array of height
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_area: i32 = 0;

        while left < right {
            max_area = max_area.max(((right - left) as i32) * height[left].min(height[right]));
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
    #[test]
    fn test_two() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
    #[test]
    fn test_three() {
        assert_eq!(Solution::max_area(vec![2, 3, 4, 5, 18, 17, 6]), 17);
    }
}
