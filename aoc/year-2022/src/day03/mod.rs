const LOW: &str = "abcdefghijklmnopqrstuvwxyz";

const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn calculate() {
    let sum = include_str!("data.txt")
        .split('\n')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let len = s.len();

            if len % 2 != 0 {
                panic!("Invalid data")
            }

            let right = &s[len / 2..];

            let mut index: i32 = -1;

            for i in s.chars().take(len / 2) {
                if let Some(value) = right.find(i) {
                    index = value as i32;
                }
            }
            if index == -1 {
                panic!("not found")
            }
            let alpha = right.chars().nth(index as usize);

            let mut priority: usize = 0;

            if let Some(v) = alpha {
                if let Some(l) = LOW.find(v) {
                    priority = l + 1;
                } else if let Some(u) = UPPER.find(v) {
                    priority = u + 27;
                }
            }

            priority
        })
        .sum::<usize>();

    println!("{:?}", sum);

    // TODO part 2

    // let sum_again = include_str!("data.txt")
    //     .split('\n')
    //     .filter(|s| !s.is_empty())
    //     .collect::<Vec<&str>>();
    //
    // if sum_again.len() % 3 != 0 {
    //     panic!("Invalid data");
    // }
    //
    // for (k, v) in sum_again.iter().step_by(3).enumerate() {
    //
    //
    // }
}
