mod utils;

use std::cmp::min;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const PI: f64 = 3.14;

// Fix position of donut and camera in space:
const R1: f64 = 1.0;
const R2: f64 = 2.0;
const K2: f64 = 5.0; // Distance from camera to donut

const theta_step: f64 = 0.017;
const phi_step: f64 = 0.005;

#[wasm_bindgen]
pub struct Screen {
    w: usize,
    h: usize,
    K1: f64, // Distance from camera
    screen: Vec<(u8, u8, u8, u8)>,
}

#[wasm_bindgen]
impl Screen {
    pub fn new(w: usize, h: usize) -> Screen{
        let screen = vec![(0, 0, 0, 255); w * h];
        Screen {
            w,
            h,
            K1: (min(w, h) as f64) * K2 * 3.0 / (8.0 * (R1 + R2)),
            screen,
        }
    }

    pub fn change_size(&mut self, w: usize, h: usize) {
        self.w = w;
        self.h = h;
        self.K1 = (min(w, h) as f64) * K2 * 3.0 / (8.0 * (R1 + R2));
        self.screen = vec![(0, 0, 0, 255); w * h];
    }

    pub fn draw_circle(&mut self) {
        let mut theta = 0.0;
        while theta < 2.0 * PI {
            let costheta = theta.cos();
            let sintheta = theta.sin();

            let phi = 0.0f64;
            let cosphi = phi.cos();
            let sinphi = phi.sin();

            let circlex = 2.0 + costheta;
            let circley = sintheta;

            let x = circlex * cosphi;
            let y = circley;
            let ooz = 1.0 / 5.0;
            let xp = (self.w as f64 / 2.0 + (self.K1 * ooz * x)) as usize;
            let yp = (self.h as f64 / 2.0 + (self.K1 * ooz * y)) as usize;
            self.screen[self.w * yp + xp] = (128, 128, 128, 255);
            theta += theta_step;
        }
    }

    pub fn draw_donut(&mut self, A: f64, B: f64, r: u8, g: u8, b: u8, a: u8) {
        let cosA = A.cos();
        let sinA = A.sin();
        let cosB = B.cos();
        let sinB = B.sin();
        let mut zbuffer = vec![0.0; self.w * self.h];
        let mut theta: f64 = 0.0;
        self.screen.clear();
        self.screen.resize(self.w * self.h, (0, 0, 0, 255));
        while theta < 2.0 * PI {
            let costheta = theta.cos();
            let sintheta = theta.sin();
            let mut phi: f64 = 0.0;
            while phi < 2.0 * PI {
                let cosphi = phi.cos();
                let sinphi = phi.sin();

                let circlex = 2.0 + costheta;

                let cosAsintheta = cosA * sintheta;
                let sinAsintheta = sinA * sintheta;
                let cosAsinphi = cosA * sinphi;
                let sinAsinphi = sinA * sinphi;
                let sinBcosphi = sinB * cosphi;
                let costhetasinphi = costheta * sinphi;

                let x = circlex * (cosB * cosphi + sinAsinphi * sinB) - cosAsintheta * sinB;
                let y = circlex * (sinB * cosphi - sinAsinphi * cosB) + cosAsintheta * cosB;
                let z = K2 + cosAsinphi * circlex + sinAsintheta;
                // let x = circlex * cosphi;
                // let y = circley;
                // let z = K2 + circlex * sinphi;
                let ooz = 1.0 / z;
                
                let xp = (self.w as f64 / 2.0 + (self.K1 * ooz * x)) as usize;
                let yp = (self.h as f64 / 2.0 + (self.K1 * ooz * y)) as usize;

                // let L = sintheta - sinphi * costheta;
                let L = sinBcosphi * costheta - cosA * costhetasinphi
                    - sinAsintheta + cosB * (cosA * sintheta - costhetasinphi * sinA);
                if L > -0.5 {
                    let idx = self.w * yp + xp;
                    if ooz > zbuffer[idx] {
                        zbuffer[idx] = ooz;
                        let L = L / 2.0f64.sqrt();
                        self.screen[idx] = ((r as f64 * L) as u8, (g as f64 * L) as u8, (b as f64 * L) as u8, a);
                    }
                }
                phi += phi_step;
            }
            theta += theta_step;
        }
    }

    pub fn get_screen(&self) -> *const (u8, u8, u8, u8) {
        self.screen.as_ptr()
    }

    pub fn get_K1(&self) -> f64 {
        self.K1
    }
}

