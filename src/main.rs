use std::str::FromStr;

use crate::shapes::{circle::Circle, collisions::Collidable, rect::Rect};
use anyhow::Result;
use shapes::collisions::{Contains, Points};

mod shapes;

#[derive(Debug)]
enum Shape {
    Rectx(Rect),
    Circle(Circle),
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (shape, data) = s.split_once(" ").unwrap_or(("", ""));
        match shape {
            "rect" => return Ok(Shape::Rectx(Rect::from_str(data)?)),
            "circle" => return Ok(Shape::Circle(Circle::from_str(data)?)),
            _ => return Err(anyhow::anyhow!("Invalid shape")),
        }
    }
}

impl Points for &Shape {
    fn points(&self) -> shapes::collisions::PointIter {
        match self {
            Shape::Rectx(r) => return r.points(),
            Shape::Circle(c) => return c.points(),
        }
    }
}

impl Contains for &Shape {
    fn contains_point(&self, point: (f64, f64)) -> bool {
        match self {
            Shape::Rectx(r) => return r.contains_point(point),
            Shape::Circle(c) => return c.contains_point(point),
        }
    }
}

fn main() -> Result<()> {
    // read from file shapes
    let file = std::fs::read_to_string("shapes")?;

    let shapes = file
        .lines()
        .filter_map(|x| x.parse::<Shape>().ok())
        .collect::<Vec<_>>();

    shapes
        .iter()
        .skip(1)
        .zip(shapes.iter().take(shapes.len() - 1))
        .filter(|(x, y)| x.collide(y))
        .for_each(|(x, y)| println!("{:?} collides with {:?}", x, y));

    return Ok(());
}
