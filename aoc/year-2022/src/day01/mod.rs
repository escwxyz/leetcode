//TODO improve

#[allow(dead_code)]
pub fn calculate() -> (u32, u32) {
    let mut array = include_str!("data.txt")
        .split("\n\n")
        .collect::<Vec<_>>()
        .iter()
        .map(|&s| match s.parse::<u32>() {
            Ok(value) => value,
            _ => s
                .split('\n')
                .map(|str| match str.parse::<u32>() {
                    Ok(value) => value,
                    _ => 0,
                })
                .sum::<u32>(),
        })
        .collect::<Vec<_>>();

    // let str = include_str!("data.txt").split('\n').collect::<Vec<_>>();
    //
    // let mut array: Vec<u32> = Vec::new();
    //
    // let mut sum = 0;
    //
    // for i in str.iter() {
    //     match i.parse::<u32>() {
    //         Ok(value) => {
    //             sum += value;
    //         }
    //         _ => {
    //             array.push(sum);
    //             sum = 0;
    //             continue;
    //         }
    //     }
    // }
    array.sort();

    let len = array.len();

    if len == 0 {
        (0, 0)
    } else if len < 3 {
        (array[len - 1], 0)
    } else {
        (
            array[len - 1],
            array[len - 1] + array[len - 2] + array[len - 3],
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day01() {
        let (most, top_three) = calculate();
        assert_eq!(69912, most);
        assert_eq!(208180, top_three);
    }
}
