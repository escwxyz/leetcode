#[allow(dead_code)]
pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let mut arr_a = Vec::from(a);
    let mut arr_b = Vec::from(b);
    arr_a.sort();
    arr_b.sort();
    let min = arr_a[arr_a.len() - 1];
    let max = arr_b[0];
    let mut result: Vec<i32> = Vec::new();

    for i in min..=max {
        let arr_aa = arr_a.iter().filter(|&x| i % x == 0).collect::<Vec<_>>();
        let arr_bb = arr_b.iter().filter(|&y| y % i == 0).collect::<Vec<_>>();
        if arr_aa.len() == arr_a.len() && arr_b.len() == arr_bb.len() {
            result.push(i);
        }
    }
    result.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(get_total_x(&[2, 4], &[16, 32, 96]), 3);
    }
}
