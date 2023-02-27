const LOW: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

const UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[allow(dead_code)]
fn caesar_cipher(s: &str, k: i32) -> String {
    s.chars()
        .map(|s| {
            let r = LOW.iter().enumerate().find(|&(_, &a)| a == s);
            let u = UPPER.iter().enumerate().find(|&(_, &a)| a == s);
            if let Some((p, _)) = r {
                match p + ((k % 26) as usize) {
                    n if n <= 25 => LOW[n],

                    n if n > 25 => LOW[n - 26],

                    _ => panic!("Error"),
                }
            } else if let Some((p, _)) = u {
                match p + ((k % 26) as usize) {
                    n if n <= 25 => UPPER[n],

                    n if n > 25 => UPPER[n - 26],

                    _ => panic!("Error"),
                }
            } else {
                s
            }
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(caesar_cipher("middle-Outz", 2), String::from("okffng-Qwvb"));
    }

    #[test]
    fn test_two() {
        assert_eq!(caesar_cipher("159357lcfd", 98), String::from("159357fwzx"));
    }
}
