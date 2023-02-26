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
