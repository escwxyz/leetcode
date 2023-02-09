mod day01;
mod day02;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day01() {
        let (most, top_three) = day01::calculate();

        assert_eq!(69912, most);

        assert_eq!(208180, top_three);
    }

    #[test]
    fn test_day02() {
        let result = day02::calculate();

        assert_eq!((12855, 13726), result);
    }
}
