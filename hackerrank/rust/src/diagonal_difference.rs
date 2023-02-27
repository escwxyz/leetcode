#[allow(dead_code)]
pub fn diagonal_diff(arr: &[Vec<i32>]) -> i32 {
    let mut a_sum: i32 = 0;
    let mut b_sum: i32 = 0;

    let len = arr.len();

    (0..len).for_each(|i| {
        for j in 0..len {
            if i == j {
                a_sum += arr[i][j];
            }
            if (i + j) == len - 1 {
                b_sum += arr[i][j];
            }
        }
    });

    (a_sum - b_sum).abs()
}
