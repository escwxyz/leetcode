mod diagonal_difference;
mod lonely_integer;
mod mini_max_sum;
mod plus_minus;
// mod sock_merchant;
mod between_two_sets;
mod divisible_sum_pairs;
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

    #[test]
    fn test_between_two_sets() {
        assert_eq!(between_two_sets::get_total_x(&[2, 4], &[16, 32, 96]), 3);
    }

    #[test]
    fn test_divisible_sum_pairs() {
        assert_eq!(divisible_sum_pairs::divisible_sum_pairs(
            6,
            3,
            &[1, 3, 2, 6, 1, 2]
        ), 5);
    }
}
