use ggez::mint::Vector4;
use crate::physics::particles::Particle;
use crate::physics::utils::mod_f32;

pub struct ParticleContainer {
    pub boundaries: Vector4<f32>,
    pub demon_looking: bool,
}

impl ParticleContainer {
    pub fn new(boundaries: Vector4<f32>) -> ParticleContainer {
        ParticleContainer { boundaries, demon_looking: false }
    }

    pub fn collision(&mut self, particle: &mut Particle) {
        let middle: f32 = (self.boundaries.w - self.boundaries.x) / 2.0;

        // Collision with left-most wall.
        if particle.radius >= mod_f32(self.boundaries.x - particle.position.x) {
            particle.position.x = particle.radius;
            particle.velocity.x *= -1.0;
        }

        // Collision with right-most wall.
        if particle.radius >= mod_f32(self.boundaries.w - particle.position.x) {
            particle.position.x = self.boundaries.w - particle.radius;
            particle.velocity.x *= -1.0;
        }

        // Collision with ceiling.
        if particle.radius >= mod_f32(self.boundaries.y - particle.position.y) {
            particle.position.y = particle.radius;
            particle.velocity.y *= -1.0;
        }

        // Collision with ground.
        if particle.radius >= mod_f32(self.boundaries.z - particle.position.y) {
            particle.position.y = self.boundaries.z - particle.radius;
            particle.velocity.y *= -1.0;
        }

        // Collision with middle wall.
        if particle.radius >= mod_f32(middle - particle.position.x) && self.demon_looking {
            let sq: f32 = particle.velocity.x.powf(2.0) + particle.velocity.y.powf(2.0);
            let speed = sq.sqrt();

            // ->  only if speed >= 3.0
            if particle.velocity.x <= 0.0 && speed < 140.0 {
                particle.position.x += particle.radius;
                particle.velocity.x *= -1.0;
            }
            // <- only if speed >= 80.0
            else if particle.velocity.x >= 0.0 && speed > 10.0 {
                particle.position.x -= particle.radius;
                particle.velocity.x *= -1.0;
            }
        }
    }
}
