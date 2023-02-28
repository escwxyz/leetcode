#[allow(dead_code)]
/// Check how many doses one needs to jump all hurdles
///
/// * `k`: The maxium height one can jump
/// * `height`: Array of heights of hurdles
///
/// Challenge:
/// https://www.hackerrank.com/challenges/the-hurdle-race/problem

fn hurdle_race(k: i32, height: &[i32]) -> i32 {
    let max = height.iter().max();
    match max {
        Some(&h) if h > k => h - k,
        Some(&h) if h <= k => 0,
        _ => panic!("Error"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(hurdle_race(1, &[1, 2, 3, 3, 2]), 2);
    }
}
