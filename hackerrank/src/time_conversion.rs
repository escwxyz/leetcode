#[allow(dead_code)]
pub fn time_conversion(s: &str) -> String {
    let hour = &s[..2];
    let tail = &s[8..];

    match tail {
        "PM" if hour == "12" => String::from(&s[..8]),
        "PM" => match hour.parse::<u32>() {
            Ok(h) => format!("{}{}", (h + 12), &s[2..8]),
            _ => panic!("Error parsing"),
        },
        "AM" if hour != "12" => String::from(&s[..8]),
        "AM" => format!("00{}", &s[2..8]),
        _ => panic!("Error"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pm() {
        assert_eq!(time_conversion("07:05:45PM"), String::from("19:05:45"));
    }

    #[test]
    fn test_12_am() {
        assert_eq!(time_conversion("12:05:32AM"), String::from("00:05:32"));
    }

    #[test]
    fn test_12_pm() {
        assert_eq!(time_conversion("12:05:32PM"), String::from("12:05:32"));
    }
}
