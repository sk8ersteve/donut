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

const theta_step: f64 = 0.007;
const phi_step: f64 = 0.002;

#[wasm_bindgen]
pub fn gsin(x: f64) -> f64 {
    x.sin()
}

#[wasm_bindgen]
pub fn gcos(x: f64) -> f64 {
    x.cos()
}

#[wasm_bindgen]
pub struct Screen {
    w: usize,
    h: usize,
    K1: f64, // Distance from camera
    screen: Vec<(u8, u8, u8, u8)>,
    donut_color: (f64, f64, f64, u8),
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
            donut_color: (255.0, 255.0, 255.0, 255),
        }
    }

    pub fn set_donut_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.donut_color = (r as f64, g as f64, b as f64, a);
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

    pub fn draw_donut(&mut self, A: f64, B: f64) {
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
                let circley = sintheta;

                let x = circlex * (cosB * cosphi + sinA * sinB * sinphi) - circley * cosA * sinB;
                let y = circlex * (sinB * cosphi - sinA * cosB * sinphi) + circley * cosA * cosB;
                let z = K2 + cosA * circlex * sinphi + circley * sinA;
                // let x = circlex * cosphi;
                // let y = circley;
                // let z = K2 + circlex * sinphi;
                let ooz = 1.0 / z;
                
                let xp = (self.w as f64 / 2.0 + (self.K1 * ooz * x)) as usize;
                let yp = (self.h as f64 / 2.0 + (self.K1 * ooz * y)) as usize;

                // let L = sintheta - sinphi * costheta;
                let L = cosphi * costheta * sinB - cosA * costheta * sinphi
                    - sinA * sintheta + cosB * (cosA * sintheta - costheta * sinA * sinphi);
                if L > 0.0 {
                    let idx = self.w * yp + xp;
                    if ooz > zbuffer[idx] {
                        zbuffer[idx] = ooz;
                        let L = L / 2.0f64.sqrt();
                        // let L = (255.0 * L / 2.0f64.sqrt()) as u8;
                        let (r, g, b, a) = self.donut_color;
                        self.screen[idx] = ((r * L) as u8, (g * L) as u8, (b * L) as u8, a);
                        // self.screen[idx] = (L, L, L, 255);
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

