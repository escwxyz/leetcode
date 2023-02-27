// TODO improve
#![allow(dead_code)]
pub fn calculate() -> (u32, u32) {
    let s = include_str!("data.txt")
        .split('\n')
        .filter(|s| !s.is_empty());

    let s1 = s.clone();

    let s2 = s.clone();

    let one = s1
        .map(|s| match s.split_once(',') {
            Some((first, second)) => {
                if let Some((min, max)) = first.split_once('-') {
                    if let Some((s_min, s_max)) = second.split_once('-') {
                        let amin = min.parse::<u32>().unwrap();
                        let amax = max.parse::<u32>().unwrap();
                        let bmin = s_min.parse::<u32>().unwrap();
                        let bmax = s_max.parse::<u32>().unwrap();

                        (amax >= bmax && amin <= bmin || amax <= bmax && amin >= bmin).into()
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            _ => 0,
        })
        .sum::<u32>();

    let two = s2
        .map(|s| match s.split_once(',') {
            Some((first, second)) => {
                if let Some((min, max)) = first.split_once('-') {
                    if let Some((s_min, s_max)) = second.split_once('-') {
                        let amin = min.parse::<u32>().unwrap();
                        let amax = max.parse::<u32>().unwrap();
                        let bmin = s_min.parse::<u32>().unwrap();
                        let bmax = s_max.parse::<u32>().unwrap();

                        (!(amax < bmin || bmax < amin)).into()
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            _ => 0,
        })
        .sum::<u32>();

    (one, two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day04() {
        let result = calculate();
        assert_eq!((475, 825), result);
    }
}
