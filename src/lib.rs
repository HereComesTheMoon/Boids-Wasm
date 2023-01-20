mod utils;
// use std::mem::swap;
use rand::{thread_rng, Rng};
// use web_sys;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!(
        "The industrial revolution and its consequences have been a disaster for the human race. {}",
        name
    ));
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct Parameters {
    number_boids: usize,
    width: u32,
    height: u32,
    border: f32,
    speed_limit: f32,
    separation_min_distance: f32,
    cohesion: f32,
    separation: f32,
    alignment: f32,
    visual_range: f32,
    mouse_position: (f32, f32),
    mouse_interactivity: f32,
}

#[wasm_bindgen]
pub struct Swarm {
    p: Parameters,
    x: Vec<f32>,
    y: Vec<f32>,
    dx: Vec<f32>,
    dy: Vec<f32>,
    dists: Vec<Vec<f32>>,
}

impl Parameters {
    const COHESION_MIN: f32 = 0.;
    const COHESION_MAX: f32 = 0.01;
    const SEPARATION_MIN: f32 = 0.;
    const SEPARATION_MAX: f32 = 0.1;
    const ALIGNMENT_MIN: f32 = 0.;
    const ALIGNMENT_MAX: f32 = 0.08;
    const VISUAL_RANGE_MIN: f32 = 0.;
    const VISUAL_RANGE_MAX: f32 = 150.;

    pub fn new(width: u32, height: u32) -> Self {
        Parameters {
            number_boids: 200,
            width,
            height,
            separation: 0.1,
            separation_min_distance: 25.0,
            alignment: 0.04,
            cohesion: 0.005,
            visual_range: 75.0,
            speed_limit: 15.,
            border: 250.,
            mouse_position: (0., 0.),
            mouse_interactivity: 1.0,
        }
    }

    // Interpolate linearly between min and max. Input should be float between 0. and 1.
    pub fn interpolate(min: f32, max: f32, val: f32) -> f32 {
        min + val * (max - min)
    }
}

impl Swarm {
    fn dists(&mut self) {
        for j in 0..self.x.len() {
            for i in j + 1..self.x.len() {
                self.dists[j][i] = dist((self.x[i], self.y[i]), (self.x[j], self.y[j]));
                self.dists[i][j] = self.dists[j][i];
            }
        }
    }

    fn cohese(&mut self) {
        for j in 0..self.x.len() {
            let mut coherence = (0.0, 0.0);
            let mut counter = 0.0;
            for i in 0..self.x.len() {
                if self.dists[i][j] == 0.0 {
                    continue;
                }
                if self.dists[i][j] > self.p.visual_range {
                    continue;
                }
                counter += 1.0;
                coherence.0 += self.x[i];
                coherence.1 += self.y[i];
            }
            if 0.0 < counter {
                coherence = (coherence.0 / counter, coherence.1 / counter);
                self.dx[j] += (coherence.0 - self.x[j]) * self.p.cohesion;
                self.dy[j] += (coherence.1 - self.y[j]) * self.p.cohesion;
            }
        }
    }

    fn separate(&mut self) {
        for j in 0..self.x.len() {
            let mut separation = (0.0, 0.0);
            for i in 0..self.x.len() {
                if i == j {
                    continue;
                }
                if self.p.separation_min_distance < self.dists[i][j] {
                    continue;
                }

                separation.0 += Parameters::interpolate(0., 1., (self.p.separation_min_distance - self.dists[i][j]) / self.p.separation_min_distance)  * (self.x[j] - self.x[i]);
                separation.1 += Parameters::interpolate(0., 1., (self.p.separation_min_distance - self.dists[i][j]) / self.p.separation_min_distance)  * (self.y[j] - self.y[i]);
            }
            self.dx[j] += separation.0 * self.p.separation;
            self.dy[j] += separation.1 * self.p.separation;
        }
    }

    fn align(&mut self) {
        for j in 0..self.x.len() {
            let mut alignment = (0.0, 0.0);
            let mut counter = 0.0;
            for i in 0..self.x.len() {
                if self.p.visual_range < self.dists[i][j] {
                    continue;
                }
                counter += 1.0;
                alignment.0 += self.dx[i];
                alignment.1 += self.dy[i];
            }
            if 0.0 < counter {
                alignment = (alignment.0 / counter, alignment.1 / counter);
                self.dx[j] += (alignment.0 - self.dx[j]) * self.p.alignment;
                self.dy[j] += (alignment.1 - self.dy[j]) * self.p.alignment;
            }
        }
    }

    fn adjust_speed(&mut self) {
        for j in 0..self.x.len() {
            let speed = (self.dx[j] * self.dx[j] + self.dy[j] * self.dy[j]).sqrt();
            if self.p.speed_limit < speed {
                self.dx[j] = (self.dx[j] / speed) * self.p.speed_limit;
                self.dy[j] = (self.dy[j] / speed) * self.p.speed_limit;
            } else if self.p.speed_limit / 2. < speed {
                self.dx[j] *= 1.002;
                self.dy[j] *= 1.002;
            } else if self.p.speed_limit / 4. < speed {
                self.dx[j] *= 1.005;
                self.dy[j] *= 1.005;
            } else {
                self.dx[j] *= 1.01;
                self.dy[j] *= 1.01;
            }
        }
    }

    fn stay_in_bounds(&mut self) {
        for j in 0..self.x.len() {
            let scale = 3.;

            fn ramp(t: f32) -> f32 {
                if t < 0. {
                    0.
                } else {
                    f32::min(t, 1.0)
                }
            }

            self.dx[j] += scale * ramp((self.p.border - self.x[j]) / self.p.width as f32);
            self.dy[j] += scale * ramp((self.p.border - self.y[j]) / self.p.height as f32);
            self.dx[j] -= scale
                * ramp((self.x[j] + self.p.border - self.p.width as f32) / self.p.width as f32);
            self.dy[j] -= scale
                * ramp((self.y[j] + self.p.border - self.p.height as f32) / self.p.height as f32);
        }
    }

    fn interact_with_mouse(&mut self) {
        const scaling: f32 = 0.002;

        for i in 0..self.p.number_boids {
            if dist(self.p.mouse_position, (self.x[i], self.y[i])) < self.p.visual_range * 4. {
                self.dx[i] += self.p.mouse_interactivity * scaling * (self.p.mouse_position.0 - self.x[i]);
                self.dy[i] += self.p.mouse_interactivity * scaling * (self.p.mouse_position.1 - self.y[i]);
            }
        }
    }
}

#[wasm_bindgen]
impl Swarm {
    pub fn tick(&mut self) {
        if self.p.speed_limit == 0. { return; }
        for k in 0..self.x.len() {
            self.x[k] += self.dx[k];
            self.y[k] += self.dy[k];
        }

        self.dists();

        self.cohese();
        self.separate();
        self.adjust_speed();
        self.align();
        self.interact_with_mouse();
        self.stay_in_bounds();
    }

    pub fn new(width: u32, height: u32) -> Swarm {
        let p = Parameters::new(width, height);
        let number_boids = p.number_boids;
        let speed_limit = p.speed_limit;
        let border = p.border;

        let mut x = vec![0_f32; number_boids];
        let mut y = vec![0_f32; number_boids];
        let mut dx = vec![0_f32; number_boids];
        let mut dy = vec![0_f32; number_boids];
        let dists = vec![vec![0_f32; number_boids]; number_boids];

        for i in 0..number_boids {
            x[i] = thread_rng().gen_range(border..width as f32 + border);
            y[i] = thread_rng().gen_range(border..height as f32 + border);
            dx[i] = thread_rng().gen_range(-speed_limit..speed_limit);
            dy[i] = thread_rng().gen_range(-speed_limit..speed_limit);
        }

        Swarm {
            p,
            x,
            y,
            dx,
            dy,
            dists,
        }
    }

    pub fn width(&self) -> u32 {
        self.p.width
    }

    pub fn height(&self) -> u32 {
        self.p.height
    }

    pub fn number_boids(&self) -> u32 {
        assert!(
            self.p.number_boids == self.x.len()
                && self.p.number_boids == self.y.len()
                && self.p.number_boids == self.dx.len()
                && self.p.number_boids == self.dy.len()
        );
        self.p.number_boids as u32
    }

    pub fn x(&self) -> *const f32 {
        self.x.as_ptr()
    }

    pub fn y(&self) -> *const f32 {
        self.y.as_ptr()
    }

    pub fn dx(&self) -> *const f32 {
        self.dx.as_ptr()
    }

    pub fn dy(&self) -> *const f32 {
        self.dy.as_ptr()
    }
}

#[wasm_bindgen]
impl Swarm {
    pub fn set_width(&mut self, width: u32) {
        self.p.width = width
    }
    pub fn set_height(&mut self, height: u32) {
        self.p.height = height
    }
    pub fn set_border(&mut self, border: f32) {
        self.p.border = border
    }
    pub fn set_separation_min_distance(&mut self, separation_min_distance: f32) {
        self.p.separation_min_distance = separation_min_distance
    }
    pub fn set_cohesion(&mut self, cohesion: f32) {
        self.p.cohesion =
            Parameters::interpolate(Parameters::COHESION_MIN, Parameters::COHESION_MAX, cohesion);
    }
    pub fn set_separation(&mut self, separation: f32) {
        self.p.separation = Parameters::interpolate(
            Parameters::SEPARATION_MIN,
            Parameters::SEPARATION_MAX,
            separation,
        );
    }
    pub fn set_alignment(&mut self, alignment: f32) {
        self.p.alignment = Parameters::interpolate(
            Parameters::ALIGNMENT_MIN,
            Parameters::ALIGNMENT_MAX,
            alignment,
        );
    }
    pub fn set_visual_range(&mut self, vision_range: f32) {
        self.p.visual_range = Parameters::interpolate(
            Parameters::VISUAL_RANGE_MIN,
            Parameters::VISUAL_RANGE_MAX,
            vision_range,
        );
    }
    pub fn set_speed_limit(&mut self, speed_limit: f32) {
        self.p.speed_limit = speed_limit
    }
    pub fn set_mouse_interactivity(&mut self, interactivity: f32) {
        self.p.mouse_interactivity = interactivity
    }
    pub fn set_mouse_position(&mut self, x: f32, y: f32) {
        self.p.mouse_position = (x, y);
    }
}


fn dist(p: (f32, f32), q: (f32, f32)) -> f32 {
    ((p.0 - q.0).powi(2) + (p.1 - q.1).powi(2)).sqrt()
}
