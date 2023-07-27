/*
Zero Area Checks/any_shape_zero_area                                                                             
                        time:   [33.292 µs 34.721 µs 36.571 µs]
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe
Zero Area Checks/any_circle_zero_area                                                                             
                        time:   [8.3944 µs 8.5007 µs 8.6525 µs]
Found 15 outliers among 100 measurements (15.00%)
  5 (5.00%) high mild
  10 (10.00%) high severe
Zero Area Checks/any_rectangle_zero_area                                                                             
                        time:   [7.4967 µs 7.6540 µs 7.8704 µs]
Found 16 outliers among 100 measurements (16.00%)
  3 (3.00%) high mild
  13 (13.00%) high severe
*/

pub trait Shape {
    fn area(&self) -> f64;
    fn description(&self) -> &str; // used to inspect types during testing
}

#[derive(Debug, Clone)]
pub struct Circle {
    radius: f64,
}
impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle { radius }
    }
    pub fn random() -> Circle {
        Circle {
            radius: rand::random::<f64>() + 1.0,
        }
    }
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
    fn description(&self) -> &str {
        "circle"
    }
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    width: f64,
    height: f64,
}
impl Rectangle {
    pub fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
    pub fn random() -> Rectangle {
        Rectangle {
            width: rand::random::<f64>() + 1.0,
            height: rand::random::<f64>() + 1.0,
        }
    }
}
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn description(&self) -> &str {
        "rectangle"
    }
}

// TODO: Write functions that take vectors of shapes and decide if any of the shapes have a area equal to zero. 
// We want to test dynamic dispatch in Rust, so we want functions that work on vectors of a single type (both Circle and Rectangle), and one that works on a vector of any values that implement Shape.
pub fn any_circle_zero_area(shapes: &Vec<Box<Circle>>) -> bool {
    shapes.iter().any(|circle| circle.area() == 0.0)
}

pub fn any_rectangle_zero_area(shapes: &Vec<Box<Rectangle>>) -> bool {
    shapes.iter().any(|rectangle| rectangle.area() == 0.0)
}

pub fn any_shape_zero_area(shapes: &Vec<Box<dyn Shape>>) -> bool {
    shapes.iter().any(|shape| shape.area() == 0.0)
}

// TODO: generate all three versions of Vec<Box<???>> for testing
// generate 2*n Circles
pub fn make_circle_vec(n: usize) -> Vec<Box<Circle>> {
    let mut v: Vec<Box<Circle>> = Vec::new();
    for _ in 0..2 * n {
        v.push(Box::new(Circle::random()));
    }
    v
}

// generate 2*n Rectangles
pub fn make_rectangle_vec(n: usize) -> Vec<Box<Rectangle>> {
    let mut v: Vec<Box<Rectangle>> = Vec::new();
    for _ in 0..2 * n {
        v.push(Box::new(Rectangle::random()));
    }
    v
}

// generate n Circles and n Rectangles
pub fn make_mixed_vec(n: usize) -> Vec<Box<dyn Shape>> {
    let mut v: Vec<Box<dyn Shape>> = Vec::new();
    for _ in 0..n {
        v.push(Box::new(Circle::random()));
        v.push(Box::new(Rectangle::random()));
    }
    v
}
