// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

use std::ops::Add;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64
}

impl Point {
    fn new(x: i8, y: i8) -> Point {
        Point { x: x as f64, y: y as f64}
    }

    fn magnitude(&self) -> f64 {
        (self.x.powi(2)+self.y.powi(2)).sqrt()
    }

    fn dist(&self, another: &Point) -> f64 {
        ((self.x - another.x).powi(2) + (self.y - another.y).powi(2)).sqrt()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, another: Point) -> Self {
        Point { x: self.x+another.x, y: self.y+another.y }
    }
}

pub struct Polygon {
    points: Vec<Point>
}

impl Polygon {
    fn new() -> Self{
        Polygon{points: vec![]}
    }

    fn add_point(&mut self, p: Point){
        self.points.push(p);
    }

    fn left_most_point(&self) -> Option<Point>{
        if self.points.len()==0 {
            return None
        }
        let mut point = self.points[0];
        for p in &self.points {
            if p.x < point.x {
                point = *p;
            }
        }
        Some(point)
    }
    fn iter(&self) -> impl Iterator<Item = &Point> {
        self.points.iter()
    }
    fn length(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }

        let mut result = 0.0;
        let mut last_point = self.points[0];
        for point in &self.points[1..] {
            result += last_point.dist(point);
            last_point = *point;
        }
        result += last_point.dist(&self.points[0]);
        result
    }
}



pub struct Circle {
    p: Point,
    r: i8
}

impl Circle {
    fn new(p: Point, r: i8) -> Self {
        Circle{p,r}
    }

    pub fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * f64::from(self.r)
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl From<Polygon> for Shape {
    fn from(poly: Polygon) -> Self {
        Shape::Polygon(poly)
    }
}

impl From<Circle> for Shape {
    fn from(circle: Circle) -> Self {
        Shape::Circle(circle)
    }
}

impl Shape {
    pub fn perimeter(&self) -> f64 {
        match self {
            Shape::Polygon(poly) => poly.length(),
            Shape::Circle(circle) => circle.circumference(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(round_two_digits(p1.dist(&p2)), 5.00);
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

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {

}