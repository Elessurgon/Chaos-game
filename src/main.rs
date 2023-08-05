mod chaos;

use std::f64::consts::PI;

use anyhow::{Context, Result};
use chaos::simulation::to_image;
use chaos::simulation::{Point, Simulation};
use clap::{Args, Parser};
use colored::Colorize;
use nalgebra::{Vector, Vector2};
#[derive(Parser, Debug)]
struct Cli {
    #[arg(short = 'x')]
    x: usize,
    #[arg(short = 'y')]
    y: usize,
    #[clap(flatten)]
    pts: Points_Group,
    #[arg(short = 'd', long = "dist")]
    prop: i32,
    #[arg(short = 'i', long = "iter")]
    iters: i128,
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
struct Points_Group {
    #[clap(short = 'e', long = "equidistant")]
    pts: Option<i32>,
    #[clap(num_args = 1.., short='p', long="points")]
    coordinates: Option<Vec<usize>>,
}

fn run_cli() {
    let args = Cli::parse();
    eprint!("{:#?}", args);

    let mut vs: Vec<Point> = Vec::new();
    match args.pts.coordinates {
        Some(v) => {
            vs = v
                .chunks(2)
                .map(|x| Point { x: x[0], y: x[1] })
                .collect::<Vec<_>>();
        }
        None => {
            let n = args.pts.pts.unwrap();
            for i in 1..n + 1 {
                vs.push(Point {
                    x: ((args.x as f64 / 3.0) as f64 * (((2.0 * PI * i as f64) / n as f64).cos())
                        + (args.x / 2) as f64) as usize,
                    y: ((args.y as f64 / 3.0) as f64 * (((2.0 * PI * i as f64) / n as f64).sin())
                        + (args.y / 2) as f64) as usize,
                })
            }
        }
    }

    eprint!("{:#?}", vs);
    let mut sim = Simulation::new(args.x as usize, args.y as usize, vs, args.prop);
    sim.run(args.iters, &Vector2::new(0, 0));
    to_image(sim.mat)
}

fn main() {
    run_cli();
}
