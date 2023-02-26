mod diagonal_difference;
mod lonely_integer;
mod mini_max_sum;
mod plus_minus;
// mod sock_merchant;
mod time_conversion;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lonly_integer() {
        assert_eq!(lonely_integer::lonely_integer(&[1, 1, 2]), 2);
        assert_eq!(lonely_integer::lonely_integer(&[0, 0, 1, 2, 1]), 2);
    }

    #[test]
    fn test_time_conversion() {
        assert_eq!(
            time_conversion::time_conversion("07:05:45PM"),
            String::from("19:05:45")
        );

        assert_eq!(
            time_conversion::time_conversion("12:05:32AM"),
            String::from("00:05:32")
        );

        assert_eq!(
            time_conversion::time_conversion("12:05:32PM"),
            String::from("12:05:32")
        );
    }
}
