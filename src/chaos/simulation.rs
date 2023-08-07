use std::ops::{Mul, Sub};

use anyhow::Error;
use hex_color::HexColor;
use image::{ImageBuffer, RgbImage};
extern crate nalgebra as na;
use clap::Parser;
use na::{abs, ArrayStorage, Dyn, MatrixMN, OMatrix, SMatrix, VecStorage, Vector2};
use rand::{seq::SliceRandom, Rng};

type Matrix = OMatrix<f32, Dyn, Dyn>;

pub fn colors(index: usize) -> &'static str {
    let colors = vec![
        "#000000", "#FFFF00", "#1CE6FF", "#FF34FF", "#FF4A46", "#008941", "#006FA6", "#A30059",
        "#FFDBE5", "#7A4900", "#0000A6", "#63FFAC", "#B79762", "#004D43", "#8FB0FF", "#997D87",
        "#5A0007", "#809693", "#FEFFE6", "#1B4400", "#4FC601", "#3B5DFF", "#4A3B53", "#FF2F80",
        "#61615A", "#BA0900", "#6B7900", "#00C2A0", "#FFAA92", "#FF90C9", "#B903AA", "#D16100",
        "#DDEFFF", "#000035", "#7B4F4B", "#A1C299", "#300018", "#0AA6D8", "#013349", "#00846F",
        "#372101", "#FFB500", "#C2FFED", "#A079BF", "#CC0744", "#C0B9B2", "#C2FF99", "#001E09",
        "#00489C", "#6F0062", "#0CBD66", "#EEC3FF", "#456D75", "#B77B68", "#7A87A1", "#788D66",
        "#885578", "#FAD09F", "#FF8A9A", "#D157A0", "#BEC459", "#456648", "#0086ED", "#886F4C",
        "#34362D", "#B4A8BD", "#00A6AA", "#452C2C", "#636375", "#A3C8C9", "#FF913F", "#938A81",
        "#575329", "#00FECF", "#B05B6F", "#8CD0FF", "#3B9700", "#04F757", "#C8A1A1", "#1E6E00",
        "#7900D7", "#A77500", "#6367A9", "#A05837", "#6B002C", "#772600", "#D790FF", "#9B9700",
        "#549E79", "#FFF69F", "#201625", "#72418F", "#BC23FF", "#99ADC0", "#3A2465", "#922329",
        "#5B4534", "#FDE8DC", "#404E55", "#0089A3", "#CB7E98", "#A4E804", "#324E72", "#6A3A4C",
        "#83AB58", "#001C1E", "#D1F7CE", "#004B28", "#C8D0F6", "#A3A489", "#806C66", "#222800",
        "#BF5650", "#E83000", "#66796D", "#DA007C", "#FF1A59", "#8ADBB4", "#1E0200", "#5B4E51",
        "#C895C5", "#320033", "#FF6832", "#66E1D3", "#CFCDAC", "#D0AC94", "#7ED379", "#012C58",
    ];
    &colors[index % colors.len()]
}

pub fn to_image(mat: Matrix) -> Result<(), Error> {
    let (imgx, imgy) = mat.shape();
    let pb = indicatif::ProgressBar::new((imgx * imgy) as u64);
    let mut img: RgbImage = ImageBuffer::new(imgx as u32, imgy as u32);
    let phi = (1.0 + (5.0 as f64).sqrt()) / 2.0;
    for x in 0..imgx as u32 {
        for y in 0..imgy as u32 {
            let pixel = img.get_pixel_mut(x, y);

            if mat[(x as usize, y as usize)] != 0.0 {
                let col = colors(mat[(x as usize, y as usize)] as usize);
                let c = HexColor::parse(col)?;

                *pixel = image::Rgb([c.r, c.g, c.b]);
            } else {
                *pixel = image::Rgb([255, 255, 255]);
            }
            pb.inc(1);
        }
    }

    img.save("test.png").unwrap();
    Ok(())
}

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
pub struct Simulation {
    pub mat: Matrix,
    pub pts: Vec<Point>,
    pub proportion: f64,
}

impl Simulation {
    pub fn new(szx: usize, szy: usize, pts: Vec<Point>, prop: f64) -> Simulation {
        let mut mat = Matrix::zeros(szx, szy);
        for p in &pts {
            let x = &mut mat[(p.x, p.y)];
            *x = 1 as f32;
        }
        Simulation {
            mat,
            proportion: prop,
            pts: pts,
        }
    }

    fn draw(&mut self, pt: &Vector2<i32>, index: usize) {
        let x: &mut f32 = &mut self.mat[(pt[0] as usize, pt[1] as usize)];
        *x = (index + 1) as f32;
    }

    pub fn run(&mut self, iters: i128, start: &Vector2<i32>) {
        let pb = indicatif::ProgressBar::new(iters as u64);

        self.draw(&start, 0);
        let mut rp = start.clone();
        for i in 0..iters {
            let index = rand::thread_rng().gen_range(0..self.pts.len());
            // let p = self.pts.choose(&mut rand::thread_rng()).unwrap();
            let p: &Point = &self.pts[index];
            let v1 = Vector2::new(rp.x, rp.y);
            let v2 = Vector2::new(p.x as i32, p.y as i32);
            let draw_pt: Vector2<i32> = Vector2::new(
                ((v1[0] + v2[0]).abs() as f64 / self.proportion) as i32,
                ((v1[1] + v2[1]).abs() as f64 / self.proportion) as i32,
            );

            self.draw(&draw_pt, index);
            rp = draw_pt;
            pb.inc(1);
        }
    }
}
