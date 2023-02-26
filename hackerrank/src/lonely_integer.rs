use std::collections::HashSet;
#[allow(dead_code)]
pub fn lonely_integer(a: &[i32]) -> i32 {
    let mut s: HashSet<i32> = HashSet::new();

    a.iter().for_each(|v| {
        if s.contains(v) {
            s.remove(v);
        } else {
            s.insert(*v);
        }
    });

    let v = s.into_iter().collect::<Vec<_>>();

    v[0]
}
