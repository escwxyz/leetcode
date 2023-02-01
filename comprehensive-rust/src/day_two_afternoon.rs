// luhn algorithm
pub fn luhn(cc_number: &str) -> bool {
    let mut len: usize = 0;
    let mut sum: u32 = 0;

    for (k, v) in cc_number.chars().rev().filter(|&c| c != ' ').enumerate() {
        match v.to_digit(10) {
            Some(value) => {
                len += 1;
                sum += if k % 2 == 0 {
                    value
                } else {
                    value * 2 / 10 + value * 2 % 10
                };
            }
            None => return false,
        }
    }

    println!("{sum}");

    len >= 2 && sum % 10 == 0
}
