mod chaos;

use chaos::simulation::to_image;
use chaos::simulation::{Point, Simulation};
use nalgebra::Vector2;

use crate::chaos::chaos::test;

fn main() {
    let mut sim = Simulation::new(
        1000,
        1000,
        vec![
            Point { x: 800, y: 200 },
            Point { x: 200, y: 500 },
            Point { x: 850, y: 850 },
        ],
        2,
    );
    let p = &Vector2::new(0, 0);
    sim.run(5000 as i128, &p);
    to_image(sim.mat);
}
