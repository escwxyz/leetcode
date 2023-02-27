// TODO improve
#![allow(dead_code)]
pub fn calculate() -> (u32, u32) {
    let s = include_str!("data.txt")
        .split('\n')
        .collect::<Vec<_>>()
        .iter()
        .map(|s| match s.split_once(' ') {
            Some((opponent, respond)) => match opponent {
                // Rock
                "A" if respond == "X" => (1 + 3, 3),
                "A" if respond == "Y" => (2 + 6, 1 + 3),
                "A" if respond == "Z" => (3, 2 + 6),
                // Paper
                "B" if respond == "X" => (1, 1),
                "B" if respond == "Y" => (2 + 3, 2 + 3),
                "B" if respond == "Z" => (3 + 6, 3 + 6),
                // Scissor
                "C" if respond == "X" => (1 + 6, 2),
                "C" if respond == "Y" => (2, 3 + 3),
                "C" if respond == "Z" => (3 + 3, 1 + 6),

                _ => (0, 0),
            },
            _ => (0, 0),
        })
        .collect::<Vec<(u32, u32)>>();

    let mut sum_one: u32 = 0;
    let mut sum_two: u32 = 0;

    s.iter().for_each(|(x, y)| {
        sum_one += x;
        sum_two += y;
    });

    (sum_one, sum_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02() {
        let result = calculate();
        assert_eq!((12855, 13726), result);
    }
}
