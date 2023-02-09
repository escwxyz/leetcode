mod day01;
mod day02;
mod day03;
mod day04;

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
    // TODO part 2
    // #[test]
    // fn test_day03() {
    //     let result = day03::calculate();
    // }

    #[test]
    fn test_day04() {
        let result = day04::calculate();
        // part 1
        // 475
        // part 2
        //825

        assert_eq!((475, 825), result);
    }
}
