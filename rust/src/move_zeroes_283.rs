use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    /// Move zeroes to the tail of array
    ///
    /// * `nums`: Array of numbers
    ///
    /// Problem 283:
    /// https://leetcode.com/problems/move-zeroes/

    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut fast: usize = 0;
        let mut slow: usize = 0;

        while fast < nums.len() {
            if nums[fast] != 0 {
                nums[slow] = nums[fast];
                slow += 1;
            }
            fast += 1;
        }

        (0..nums.len() - slow).for_each(|i| {
            nums[slow + i] = 0;
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let mut arr = vec![0, 1, 0, 3, 12];

        Solution::move_zeroes(&mut arr);

        assert_eq!(vec![1, 3, 12, 0, 0], arr);
    }

    #[test]
    fn test_two() {
        let mut arr = vec![0];

        Solution::move_zeroes(&mut arr);

        assert_eq!(vec![0], arr);
    }
}
