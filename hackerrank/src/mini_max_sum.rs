#[allow(dead_code)]
pub fn mini_max_sum(arr: &[i32]) {
    let mut a: Vec<i64> = Vec::new();

    for v in arr.iter() {
        a.push(*v as i64);
    }
    a.sort();

    let min = a.iter().take(4).sum::<i64>();
    let max = min - a[0] + a[4];

    println!("{min} {max}");
}
