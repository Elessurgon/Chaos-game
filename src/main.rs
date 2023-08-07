mod chaos;

use std::f64::consts::PI;
use std::fmt::{self, Display};

use anyhow::{Context, Error, Result};
use chaos::simulation::to_image;
use chaos::simulation::{Point, Simulation};
use clap::{Args, Parser};
use colored::Colorize;
use nalgebra::{Vector, Vector2};
#[derive(Parser, Debug)]
struct Cli {
    #[arg(short = 'x', default_value_t = 1000)]
    x: usize,
    #[arg(short = 'y', default_value_t = 1000)]
    y: usize,
    #[clap(flatten)]
    pts: PointsGroup,
    #[arg(short = 'n', long = "numerator", default_value_t = 1.0)]
    num: f64,
    #[arg(short = 'd', long = "denominator", default_value_t = 2.0)]
    deno: f64,
    #[arg(short = 'i', long = "iter", default_value_t = 10000)]
    iters: i128,
    #[arg(long = "color", value_parser, num_args = 1..3, value_delimiter = ',', default_values_t = [255, 255, 255])]
    color: Vec<u8>,
}

#[derive(Debug, clap::Args)]
#[group(multiple = false)]
struct PointsGroup {
    #[clap(short = 'e', long = "equidistant", default_value_t = 3)]
    pts: i32,
    #[clap(num_args = 1.., short='p', long="points")]
    coordinates: Option<Vec<usize>>,
}

fn run_cli() -> Result<(), Error> {
    let args = Cli::parse();

    let mut vs: Vec<Point> = Vec::new();
    match args.pts.coordinates {
        Some(v) => {
            vs = v
                .chunks(2)
                .map(|x| Point { x: x[0], y: x[1] })
                .collect::<Vec<_>>();
        }
        None => {
            let n = args.pts.pts;
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

    let mut sim = Simulation::new(
        args.x as usize,
        args.y as usize,
        vs,
        args.num / args.deno as f64,
    );
    sim.run(
        args.iters,
        &Vector2::new((args.x / 2) as i32, (args.y / 2) as i32),
    );
    let colors: [u8; 3] = if args.color.len() == 3 {
        [args.color[0], args.color[1], args.color[2]]
    } else {
        [255, 255, 255]
    };
    to_image(sim.mat, colors)?;
    Ok(())
}

fn main() -> Result<(), Error> {
    run_cli()?;
    Ok(())
}
