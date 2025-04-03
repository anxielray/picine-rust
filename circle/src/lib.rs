#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn distance(&self, other: Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            center: Point::new(x, y),
            radius,
        }
    }

    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    pub fn intersect(&self, other: Circle) -> bool {
        let distance_between_centers = self.center.distance(other.center);
        distance_between_centers < (self.radius + other.radius)
    }
}
