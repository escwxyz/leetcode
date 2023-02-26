mod lonely_integer;
mod diagonal_difference;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lonly_integer() {
        assert_eq!(lonely_integer::lonely_integer(&[1, 1, 2]), 2);
        assert_eq!(lonely_integer::lonely_integer(&[0, 0, 1, 2, 1]), 2);
    }
}
