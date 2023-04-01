#[allow(dead_code)]
pub fn is_subsequence(s: String, t: String) -> bool {
    let mut index: i32 = -1;
    let mut result: bool = true;

    for c in s.chars() {
        index += 1;
        let i = t.chars().skip(index as usize).position(|v| v == c);
        if i.is_none() {
            result = false;
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_false() {
        assert!(
            !is_subsequence(String::from("axc"), String::from("ahbgdc"))
        );
    }

    #[test]
    fn test_true() {
        assert!(
            is_subsequence(String::from("abc"), String::from("ahbgdc"))
        );
    }
}
