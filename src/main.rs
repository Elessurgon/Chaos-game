mod chaos;

use anyhow::{Context, Result};
use chaos::simulation::to_image;
use chaos::simulation::{Point, Simulation};
use clap::Parser;
use colored::Colorize;
use nalgebra::{Vector, Vector2};
#[derive(Parser)]
struct Cli {
    #[arg(short = 'x')]
    x: usize,
    #[arg(short = 'y')]
    y: usize,
    #[clap(required = true, num_args = 1.., short='p', long="points")]
    pts: Vec<usize>,
    #[arg(short = 'd', long = "dist")]
    prop: i32,
    #[arg(short = 'i', long = "iter")]
    iters: i128,
}

fn run_cli() {
    let args = Cli::parse();
    let mut v: Vec<usize> = args.pts;
    let vs = v
        .chunks(2)
        .map(|x| Point { x: x[0], y: x[1] })
        .collect::<Vec<_>>();
    let mut sim = Simulation::new(args.x as usize, args.y as usize, vs, args.prop);
    sim.run(args.iters, &Vector2::new(0, 0));
    to_image(sim.mat)
}

fn main() {
    run_cli();
}
