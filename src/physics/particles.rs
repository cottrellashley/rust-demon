use ggez::graphics::{Color};
use ggez::mint::{Vector2};
use rand::Rng;
use crate::physics::container::{ParticleContainer};
use crate::physics::utils::random_vector;


#[derive(Debug)]
pub struct Particle {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub force: Vector2<f32>,
    pub radius: f32,
    // pub mass: f32, // For later... we now assume unit mass for all.
}


impl Particle {
    // Create a new particle with a random position and velocity.
    pub fn new(container: &ParticleContainer) -> Self {
        let mut rng = rand::thread_rng();
        let radius = 5.0;
        let x = rng.gen_range(radius..container.boundaries.w - radius);
        let y = rng.gen_range(radius..container.boundaries.z - radius);

        let velocity = random_vector(250.0, 50.0);
        Particle {
            position: Vector2 { x, y },
            velocity: velocity,
            force: Vector2 { x: 0.0, y: 0.0 },
            radius,
        }
    }

    fn update_position(&mut self, dt: f32) {
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;
    }

    fn update_velocity(&mut self, _dt: f32) {
        // Velocity remains constant; you could add acceleration here if desired.
        self.velocity.y += 9.81 * _dt;
        self.velocity.y += self.force.y * _dt;
        self.velocity.x += self.force.x * _dt;
    }

    pub fn update(&mut self, dt: f32) {
        self.update_velocity(dt);
        self.update_position(dt);
    }

    fn speed(&self) -> f32 {
        let speed: f32 = (self.velocity.x.powf(2.0) + self.velocity.y.powf(2.0)).sqrt();
        speed
    }

    pub fn kinetic_energy(&self) -> f32 {
        let speed: f32 = self.speed();
        let ke = 0.5*speed.powi(2);
        return ke
    }

    pub fn color(&self, avg: &f32) -> Color {
        let self_ke = self.kinetic_energy();
        // Normalize kinetic energy so an average particle roughly maps to 50.
        let normalized = if *avg > 0.0 { (self_ke / *avg) * 50.0 } else { 0.0 };
        let t = normalized.clamp(0.0, 100.0);

        // Define anchor colors with explicit f32 annotations.
        let blue: (f32, f32, f32)   = (0.0_f32,   0.0_f32, 255.0_f32);
        let yellow: (f32, f32, f32) = (255.0_f32, 255.0_f32,   0.0_f32);
        let orange: (f32, f32, f32) = (255.0_f32, 165.0_f32,   0.0_f32);
        let red: (f32, f32, f32)    = (255.0_f32,   0.0_f32,   0.0_f32);

        // Smoothstep function for smooth interpolation.
        let smoothstep = |edge0: f32, edge1: f32, x: f32| -> f32 {
            let factor = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
            factor * factor * (3.0 - 2.0 * factor)
        };

        // Helper closure to interpolate between two colors.
        let interpolate_color = |start: (f32, f32, f32), end: (f32, f32, f32), factor: f32| -> (u8, u8, u8) {
            let r = start.0 + (end.0 - start.0) * factor;
            let g = start.1 + (end.1 - start.1) * factor;
            let b = start.2 + (end.2 - start.2) * factor;
            (r.round() as u8, g.round() as u8, b.round() as u8)
        };

        // Map the normalized value (t) to a color.
        let (r, g, b) = if t <= 20.0 {
            // Directly return blue.
            (blue.0.round() as u8, blue.1.round() as u8, blue.2.round() as u8)
        } else if t <= 40.0 {
            let factor = smoothstep(20.0, 40.0, t);
            interpolate_color(blue, yellow, factor)
        } else if t <= 60.0 {
            let factor = smoothstep(40.0, 60.0, t);
            interpolate_color(yellow, orange, factor)
        } else if t <= 80.0 {
            let factor = smoothstep(60.0, 80.0, t);
            interpolate_color(orange, red, factor)
        } else {
            // Directly return red.
            (red.0.round() as u8, red.1.round() as u8, red.2.round() as u8)
        };

        Color::from_rgb(r, g, b)
    }
}
