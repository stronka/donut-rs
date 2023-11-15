use std::f64::consts::PI;

use linalg::Matrix;

use crate::linalg::Vector;

pub mod linalg;

// config
const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 40;
const R1: f64 = 1.;
const R2: f64 = 2.;
const K2: f64 = 5.;
const K1: f64 = 1.;

const THETA_STEP: f64 = 0.01;
const PHI_STEP: f64 = 0.01;

pub fn render(x_rot: f64, z_rot: f64) {
    let cx = x_rot.cos();
    let cz = z_rot.cos();

    let sx = x_rot.sin();
    let sz = z_rot.sin();

    let mut theta = 0.0;
    let mut phi = 0.0;

    let frame_rotation: Matrix<f64, 3, 3> = Matrix::new(
        [
            [1., 0., 0.],
            [0., cx, sx],
            [0., -sx, cx],
        ],
    ).dot(
        &Matrix::new(
            [
                [cz, sz, 0.],
                [-sz, cz, 0.],
                [0., 0., 1.],
            ]
        )
    );

    let mut zbuff: Matrix<f64, SCREEN_WIDTH, SCREEN_HEIGHT> = Matrix::zeros();

    while phi < 2. * PI {
        let cp = phi.cos();
        let sp = phi.sin();

        let phi_rotation: Matrix<f64, 3, 3> = Matrix::new(
            [
                [cp, 0., sp],
                [0., 1., 0.],
                [-sp, 0., cp],
            ],
        ).dot(&frame_rotation);


        while theta < 2. * PI {
            let ct = theta.cos();
            let st = theta.sin();

            let point: Vector<f64, 3> = Vector::new(
                [
                    R2 + R1 * ct,
                    R1 * st,
                    0.
                ]
            ).mdot(
                &phi_rotation
            ).add(
                &Vector::new(
                    [0., 0., K2],
                )
            );

            let one_over_z = point.at(2).unwrap();

            let xp: i32 = (SCREEN_WIDTH as f64 / 2. + K1 * point.at(0).unwrap() * one_over_z).round() as i32;
            let yp: i32 = (SCREEN_HEIGHT as f64 / 2. - K1 * point.at(1).unwrap() * one_over_z).round() as i32;
            let xpi: usize = xp.try_into().unwrap_or(SCREEN_WIDTH);
            let ypi: usize = yp.try_into().unwrap_or(SCREEN_HEIGHT);

            if let Some(z_val) = zbuff.at(xpi, ypi) {
                if z_val < one_over_z {
                    zbuff.set(xpi, ypi, one_over_z);
                }
            }

            theta += THETA_STEP;
        }

        theta = 0.;
        phi += PHI_STEP;
    }


    print!("\x1b[H");

    for j in 0..SCREEN_HEIGHT {
        for i in 0..SCREEN_WIDTH {
            if zbuff.at(i, j).unwrap() > 0. {
                print!("@");
            } else {
                print!("-");
            }
        }

        print!("\n");
    }


    // println!("DEBUG info:");
    // println!("lmin: {:.4}", lmin);
    // println!("lmax: {:.4}", lmax);
}