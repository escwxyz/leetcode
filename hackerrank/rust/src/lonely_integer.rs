// Challenge:
// https://www.hackerrank.com/challenges/one-week-preparation-kit-lonely-integer/problem

use std::collections::HashSet;
#[allow(dead_code)]
pub fn lonely_integer(a: &[i32]) -> i32 {
    let mut s: HashSet<i32> = HashSet::new();

    a.iter().for_each(|v| {
        if s.contains(v) {
            s.remove(v);
        } else {
            s.insert(*v);
        }
    });

    let v = s.into_iter().collect::<Vec<_>>();

    v[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        assert_eq!(lonely_integer(&[1, 1, 2]), 2);
    }
    #[test]
    fn test_02() {
        assert_eq!(lonely_integer(&[0, 0, 1, 2, 1]), 2);
    }
}
