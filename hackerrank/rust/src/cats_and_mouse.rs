#[allow(dead_code)]
fn cats_and_mouse(x: i32, y: i32, z: i32) -> String {
    let d1 = (x - z).abs();
    let d2 = (y - z).abs();

    match d1 - d2 {
        0 => String::from("Mouse C"),
        v if v > 0 => String::from("Cat B"),
        v if v < 0 => String::from("Cat A"),
        _ => panic!("ERR"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cat_a() {
        assert_eq!(cats_and_mouse(1, 6, 2), String::from("Cat A"));
    }

    #[test]
    fn test_cat_b() {
        assert_eq!(cats_and_mouse(1, 2, 3), String::from("Cat B"));
    }

    #[test]
    fn test_mouse_c() {
        assert_eq!(cats_and_mouse(1, 3, 2), String::from("Mouse C"));
    }
}
