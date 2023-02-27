#[allow(dead_code)]
/// Calculate the height of tree after `n` growth cycles
///
/// * `n`: number of growth cycles
///
/// Challenge:
/// https://www.hackerrank.com/challenges/utopian-tree/problem

fn utopian_tree(n: i32) -> i32 {
    let mut h = 1;
    for i in 1..=n {
        if i % 2 != 0 {
            h *= 2;
        } else {
            h += 1;
        }
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(utopian_tree(5), 14);
    }
}
