#[allow(dead_code)]
pub fn plus_minus(arr: &[i32]) {
    let mut positive: usize = 0;
    let mut negative: usize = 0;
    let mut zero: usize = 0;

    for v in arr.iter() {
        match *v {
            0 => zero += 1,
            _ if *v > 0 => positive += 1,
            _ => negative += 1,
        }
    }

    println!("{:.6}", positive as f32 / arr.len() as f32);
    println!("{:.6}", negative as f32 / arr.len() as f32);
    println!("{:.6}", zero as f32 / arr.len() as f32);
}
