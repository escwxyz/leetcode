mod day_one_afternoon;
mod day_three_afternoon;
mod day_two_afternoon;
mod day_two_morning;

use crate::day_one_afternoon::{Book, Library};

use crate::day_two_morning::User;

use crate::day_three_afternoon::DirectoryIterator;

fn main() -> Result<(), String> {
    println!("Day 1 afternoon");

    let mut library = Library::new();

    println!("Our library is empty: {}", library.is_empty());
    //
    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    //
    library.print_books();
    //
    match library.oldest_book() {
        Some(book) => println!("My oldest book is {book}"),
        None => println!("My library is empty!"),
    }
    //
    println!("Our library has {} books", library.len());

    println!("Day 2 morning");

    let bob = User::new(String::from("Bob"), 32, 155.2);

    println!("I am {} and my age is {}", bob.name(), bob.age());

    println!("Day 3 morning");

    let iter = DirectoryIterator::new(".")?;
    println!("files:{:#?}", iter.collect::<Vec<_>>());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_two_afternoon::luhn;
    use crate::day_two_morning::{Point, Polygon};

    // day 2

    #[test]
    fn test_weight() {
        let bob = User::new(String::from("Bob"), 32, 155.2);
        assert_eq!(bob.weight(), 155.2);
    }

    #[test]
    fn test_set_age() {
        let mut bob = User::new(String::from("Bob"), 32, 155.2);
        assert_eq!(bob.age(), 32);
        bob.set_age(33);
        assert_eq!(bob.age(), 33);
    }

    // day 2
    //
    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }
    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();

        poly.add_point(p1);
        poly.add_point(p2);

        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();

        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();

        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }
    // #[test]
    // fn test_shape_perimeters() {
    //     let mut poly = Polygon::new();
    //     poly.add_point(Point::new(12, 13));
    //     poly.add_point(Point::new(17, 11));
    //     poly.add_point(Point::new(16, 16));
    //
    //     let shapes = vec![
    //         Shape::from(poly),
    //         Shape::from(Circle::new(Point::new(10, 20), 5)),
    //     ];
    //
    //     let perimeters = shapes.iter().map(round_two_digits).collect::<Vec<_>>();
    //
    //     assert_eq!(perimeters, vec![15.48, 31.42]);
    //
    //
    // }
    // day 2
    #[test]
    fn test_luhn() {
        assert!(!luhn("")); // false
        assert!(!luhn("  ")); // false
        assert!(!luhn("foo"));
        assert!(!luhn("0"));
        assert!(!luhn("0 0 0 ")); // panicked
        assert!(luhn("4263 9826 4026 9299")); // true
        assert!(luhn("7992 7398 713")); // true
        assert!(!luhn("4223 9826 4026 9299"));
    }
}
