use std::f64::consts::PI;
use std::thread::sleep;
use std::time::Duration;

use donut_rs::render;

fn main() {
    let mut x_rot = 0.;
    let mut z_rot = 0.;

    loop {
        render(x_rot, z_rot);
        sleep(Duration::from_millis(10));

        x_rot += 0.03;
        z_rot += 0.01;
        if x_rot > 2. * PI {
            x_rot = 0.;
        }

        if z_rot > 2. * PI {
            z_rot = 0.;
        }
    }
}
