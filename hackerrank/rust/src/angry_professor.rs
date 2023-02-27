#[allow(dead_code)]
/// Check if class is to be cancelled based on the arrival number of students
///
/// * `k`: Cancellation threshhold
/// * `a`: Array of arrival times of students
///
/// Challenge:
/// https://www.hackerrank.com/challenges/angry-professor/problem

fn angry_professor(k: i32, a: &[i32]) -> String {
    let c = a.iter().filter(|&&v| v <= 0).count() as i32;

    if c >= k {
        String::from("NO")
    } else {
        String::from("YES")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancelled() {
        assert_eq!(angry_professor(3, &[-1, -3, 4, 2]), String::from("YES"));
    }

    #[test]
    fn test_go_on() {
        assert_eq!(angry_professor(2, &[0, -1, 2, 1]), String::from("NO"));
    }
}
