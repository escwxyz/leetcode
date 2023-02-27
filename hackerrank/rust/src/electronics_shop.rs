#[allow(dead_code)]
/// Get the money spent on keyboards and drives, return `-1` if impossible to buy both
///
/// * `keyboards`: Price array of keyboards
/// * `drives`: Price arry of drives
/// * `b`: Total budget
/// Challenges:
/// https://www.hackerrank.com/challenges/electronics-shop/problem

pub fn get_money_spent(keyboards: &[i32], drives: &[i32], b: i32) -> i32 {
    let a = keyboards
        .iter()
        .filter(|&&v| v < b)
        .map(|&v| {
            let d = drives.iter().filter(|&&w| w + v <= b).max();
            if let Some(&s) = d {
                v + s
            } else {
                0
            }
        })
        .collect::<Vec<_>>();

    match a.is_empty() {
        true => -1,
        _ => match a.iter().max() {
            Some(&v) => v,
            _ => -1,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(get_money_spent(&[5, 2, 8], &[3, 1], 10), 9);
    }

    #[test]
    fn test_two() {
        assert_eq!(get_money_spent(&[5], &[4], 5), -1);
    }
}
