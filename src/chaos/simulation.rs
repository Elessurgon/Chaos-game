use std::ops::{Mul, Sub};

use image::{ImageBuffer, RgbImage};
extern crate nalgebra as na;
use na::{abs, ArrayStorage, Dyn, MatrixMN, OMatrix, SMatrix, VecStorage, Vector2};
use rand::seq::SliceRandom;

type Matrix = OMatrix<f32, Dyn, Dyn>;

pub fn to_image(mat: Matrix) {
    let (imgx, imgy) = mat.shape();
    let mut img: RgbImage = ImageBuffer::new(imgx as u32, imgy as u32);

    for x in 0..imgx as u32 {
        for y in 0..imgy as u32 {
            let pixel = img.get_pixel_mut(x, y);
            if mat[(x as usize, y as usize)] == 1.0 {
                *pixel = image::Rgb([255, 255, 255]);
            }
        }
    }

    img.save("test.png").unwrap();
}

pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct Simulation {
    pub mat: Matrix,
    pub pts: Vec<Point>,
    pub proportion: i32,
}

impl Simulation {
    pub fn new(szx: usize, szy: usize, pts: Vec<Point>, prop: i32) -> Simulation {
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

    fn draw(&mut self, pt: &Vector2<i32>) {
        let x = &mut self.mat[(pt[0] as usize, pt[1] as usize)];
        *x = 1 as f32;
    }

    pub fn run(&mut self, iters: i128, start: &Vector2<i32>) {
        self.draw(&start);
        let mut rp = start.clone();
        for _ in 0..iters {
            let p = self.pts.choose(&mut rand::thread_rng()).unwrap();
            let v1 = Vector2::new(rp.x, rp.y);
            let v2 = Vector2::new(p.x as i32, p.y as i32);
            // eprint!("{:?} {:?}\n", v1, v2);
            let draw_pt: Vector2<i32> = Vector2::new(
                (v1[0] - v2[0]).abs() / self.proportion,
                (v1[1] - v2[1]).abs() / self.proportion,
            );

            self.draw(&draw_pt);
            rp = draw_pt;
        }
    }
}
