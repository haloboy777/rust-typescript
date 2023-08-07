use std::{f64::consts::PI, fmt::Display};

use super::{area::Area, collisions::Collidable, rect::Rect};

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Circle {
    pub fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        return (x - self.x).powi(2) + (y - self.y).powi(2) <= self.radius.powi(2);
    }
}

impl Collidable<Rect> for Circle {
    fn collide(&self, other: &Rect) -> bool {
        for point in other {
            if self.contains_point(point) {
                return true;
            }
        }
        return false;
    }
}
impl Collidable<Circle> for Circle {
    fn collide(&self, other: &Circle) -> bool {
        for point in other {
            if self.contains_point(point) {
                return true;
            }
        }
        return false;
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * PI;
    }
}
impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Circle({}, {}): {}", self.x, self.y, self.radius);
    }
}

pub struct CircIter {
    idx: usize,
    points: Vec<(f64, f64)>,
}

impl Iterator for CircIter {
    // this is what the iterator will produce
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.points.len() {
            return None;
        }
        let point = self.points[self.idx];
        self.idx += 1;
        return Some(point);
    }
}

impl IntoIterator for &Circle {
    type Item = (f64, f64);
    type IntoIter = CircIter;

    fn into_iter(self) -> Self::IntoIter {
        return CircIter {
            idx: 0,
            points: vec![(self.x, self.y)],
        };
    }
}
