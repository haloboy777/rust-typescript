use shapes::collisions::Collidable;

use crate::shapes::{circle::Circle, rect::Rect};

mod shapes;

fn main() {
    let rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
    };
    let rect2 = Rect {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
    };
    let circ = Circle {
        radius: 1.0,
        x: 0.0,
        y: 0.0,
    };
    let circ2 = Circle {
        radius: 3.0,
        x: 3.0,
        y: 3.0,
    };

    // rect.collide(&rect2);
    // circ.collide(&circ2);
    // rect.collide(&circ);
    println!("rect collides with rect2: {}", rect.collide(&rect2));
    println!("circ collides with circ2: {}", circ.collide(&circ2));
    println!("rect collides with circ: {}", rect.collide(&circ));
}
