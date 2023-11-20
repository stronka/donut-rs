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

const N_THETA_STEPS: usize = 50;
const PHI_STEP: f64 = 0.04;

const FLOAT_SCREEN_WIDTH: f64 = SCREEN_WIDTH as f64;
const FLOAT_SCREEN_HEIGHT: f64 = SCREEN_HEIGHT as f64;

const LUMINANCE_LEVEL: [char; 9] = [
    '.',
    ',',
    '-',
    '~',
    ':',
    ';',
    '=',
    '#',
    '@',
];


pub fn render(x_rot: f64, z_rot: f64, theta_sin_cos: &[(f64, f64); N_THETA_STEPS]) {
    let projection_offset = Vector::new([0., 0., K2]);
    let light_direction = Vector::new([0., 1., 1.]).normalize();

    let (sx, cx) = x_rot.sin_cos();
    let (sz, cz) = z_rot.sin_cos();

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
    let mut output: [[char; SCREEN_WIDTH]; SCREEN_HEIGHT] = [[' '; SCREEN_WIDTH]; SCREEN_HEIGHT];

    let mut phi = -PI;

    while phi < PI {
        let (sp, cp) = phi.sin_cos();

        let phi_rotation: Matrix<f64, 3, 3> = Matrix::new(
            [
                [cp, 0., sp],
                [0., 1., 0.],
                [-sp, 0., cp],
            ],
        ).dot(&frame_rotation);


        for (st, ct) in theta_sin_cos.iter().copied() {
            let point: Vector<f64, 3> = Vector::new(
                [
                    R2 + R1 * ct,
                    R1 * st,
                    0.
                ]
            ).mdot(
                &phi_rotation
            ).add(
                &projection_offset
            );

            let one_over_z = point.at(2).unwrap();

            let xp: i32 = (FLOAT_SCREEN_WIDTH / 2. + K1 * point.at(0).unwrap() * one_over_z).round() as i32;
            let yp: i32 = (FLOAT_SCREEN_HEIGHT / 2. - K1 * point.at(1).unwrap() * one_over_z).round() as i32;
            let xpi: usize = xp.try_into().unwrap_or(SCREEN_WIDTH);
            let ypi: usize = yp.try_into().unwrap_or(SCREEN_HEIGHT);

            let z_val = zbuff.at(xpi, ypi).unwrap_or(1e3);
            let mut luminance: f64 = -1.;

            if z_val < one_over_z {
                luminance = Vector::new(
                    [ct, st, 0.],
                ).mdot(
                    &phi_rotation
                ).dot(
                    &light_direction
                );
            }

            if luminance > 0. {
                zbuff.set(xpi, ypi, one_over_z);

                let luminance_index: usize = (luminance * 9.).floor() as usize;
                output[ypi][xpi] = LUMINANCE_LEVEL.get(luminance_index).copied().unwrap();
            }
        }

        phi += PHI_STEP;
    }

    render_output(&output);
}

#[inline]
fn render_output(output: &[[char; SCREEN_WIDTH]; SCREEN_HEIGHT]) {
    let mut frame: String = String::new();

    output.iter().for_each(|row| {
        row.iter().for_each(|c| frame.push(*c));
        frame.push('\n');
    });

    print!("\x1b[H{}", frame);
}


pub fn compute_theta_sin_cos() -> [(f64, f64); N_THETA_STEPS] {
    let mut result = [(0., 0.); N_THETA_STEPS];

    let mut theta = -PI;
    let step = 2. * PI / (N_THETA_STEPS as f64);

    for i in 0..N_THETA_STEPS {
        result[i] = theta.sin_cos();
        theta += step;
    }

    result
}