// Health Statics
pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        Self { name, age, weight }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn weight(&self) -> f32 {
        self.weight
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }

    pub fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
    }
}

// Points and Polygons

use std::f32::consts::PI;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn magnitude(&self) -> f64 {
        let r = (self.x.pow(2) + self.y.pow(2)) as f64;
        r.sqrt()
    }

    pub fn dist(&self, point: Point) -> f64 {
        let x = self.x - point.x;
        let y = self.y - point.y;

        let r = (x.pow(2) + y.pow(2)) as f64;

        r.sqrt()
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
#[derive(Clone)]
pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    // We must derive Copy trait for this
    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }
    //
    pub fn left_most_point(&self) -> Option<Point> {
        self.points.clone().into_iter().min_by_key(|&p| p.x)
    }

    pub fn iter(&self) -> std::slice::Iter<Point> {
        self.points.iter()
    }

    pub fn perimeter(&self) -> f32 {
        todo!();
    }
}
#[derive(Clone)]
pub struct Circle {
    point: Point,
    radius: i32,
}

impl Circle {
    pub fn new(point: Point, radius: i32) -> Self {
        Self { point, radius }
    }

    pub fn perimeter(&self) -> f32 {
        let radius = self.radius as f32;
        2.0 * PI * radius
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl From<Circle> for Shape {
    fn from(value: Circle) -> Self {
        Shape::Circle(value)
    }
}
impl From<Polygon> for Shape {
    fn from(value: Polygon) -> Self {
        Shape::Polygon(value)
    }
}
