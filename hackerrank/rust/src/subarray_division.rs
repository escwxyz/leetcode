#[allow(dead_code)]
fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    let mut count = 0;
    for i in 0..s.len() as i32 - m + 1 {
        let sum = s.iter().skip(i as usize).take(m as usize).sum::<i32>();
        if sum == d {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(birthday(&[1, 2, 1, 3, 2], 3, 2), 2);
    }

    #[test]
    fn test_two() {
        assert_eq!(birthday(&[1, 1, 1, 1, 1, 1], 3, 2), 0);
    }

    #[test]
    fn test_three() {
        assert_eq!(birthday(&[4], 4, 1), 1);
    }
}
