use std::{f64::consts::PI, fmt::Display, str::FromStr};

use super::{
    area::Area,
    collisions::{Contains, Points},
};

#[derive(Debug, Clone)]
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

impl Points for Circle {
    fn points(&self) -> super::collisions::PointIter {
        return vec![(self.x, self.y)].into();
    }
}
impl Contains for Circle {
    fn contains_point(&self, point: (f64, f64)) -> bool {
        return self.contains_point(point);
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

impl FromStr for Circle {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(anyhow::anyhow!("Invalid number of parts"));
        }
        let x = parts[0].parse::<f64>()?;
        let y = parts[1].parse::<f64>()?;
        let radius = parts[2].parse::<f64>()?;
        return Ok(Circle { x, y, radius });
    }
}
