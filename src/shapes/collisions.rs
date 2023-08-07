pub trait Collidable<T> {
    fn collide(&self, other: &T) -> bool;
    fn collides(&self, others: &[T]) -> bool {
        for other in others {
            if self.collide(other) {
                return true;
            }
        }
        return false;
    }
}

pub struct PointIter {
    idx: usize,
    points: Vec<(f64, f64)>,
}

impl Iterator for PointIter {
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

impl From<Vec<(f64, f64)>> for PointIter {
    fn from(value: Vec<(f64, f64)>) -> Self {
        return PointIter {
            points: value,
            idx: 0,
        };
    }
}

pub trait Points {
    fn points(&self) -> PointIter;
}
pub trait Contains {
    fn contains_point(&self, point: (f64, f64)) -> bool;
}

impl<T, V> Collidable<T> for V
where
    T: Points,
    V: Contains,
{
    fn collide(&self, other: &T) -> bool {
        for point in other.points() {
            if self.contains_point(point) {
                return true;
            }
        }
        return false;
    }
}
