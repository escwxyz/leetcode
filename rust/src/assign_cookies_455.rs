use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        if s.is_empty() {
            return -0;
        }

        let mut count: i32 = 0;

        let mut a: Vec<i32> = g;
        a.sort();

        let mut b: Vec<i32> = s;
        b.sort();

        let mut p = 0;
        let mut q = 0;

        while p < a.len() && q < b.len() {
            if b[q] >= a[p] {
                count += 1;
                q += 1;
                p += 1;
            } else {
                q += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2, 3], vec![1, 1]),
            1
        );
    }
    #[test]
    fn test_two() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2], vec![1, 2, 3]),
            2
        );
    }
}
