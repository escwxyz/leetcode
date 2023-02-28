#[allow(dead_code)]
/// Remove `val` from `nums` array in place, return length of updated array
///
/// * `nums`: Array to be worked on
/// * `val`: Target value to remove
///
/// Problem:
/// https://leetcode.cn/problems/remove-element/
///
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut slow: usize = 0;
    let mut fast: usize = 0;

    while fast < nums.len() {
        if nums[fast] != val {
            nums[slow] = nums[fast];
            slow += 1;
        }
        fast += 1;
    }
    slow as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let mut v = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(remove_element(&mut v, 2), 5);
    }
}
