#[allow(dead_code)]
pub fn divisible_sum_pairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut count :i32 = 0;
    for i in 0..n {
        for j in i+1..n {
            if (ar[i as usize] + ar[j as usize]) % k == 0 {
                count += 1;
            }
        }
    }
    count
}