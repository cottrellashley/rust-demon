use ggez::graphics::{self};
use ggez::mint::Vector4;
use ggez::{Context, GameResult};
use crate::physics::container::ParticleContainer;
use crate::physics::laws::{build_interaction_law, InteractionLaw, InteractionLawType};
use crate::physics::particles::Particle;

// Main state holding the collection of particles.
pub struct MainState {
    pub particles: Vec<Particle>,
    pub container: ParticleContainer,
    pub law: InteractionLawType,
    pub slider_value: f32,
    pub paused: bool,
}

impl MainState {
    pub fn new(ctx: &mut Context, num: u32, law: InteractionLawType) -> GameResult<MainState> {
        let (screen_width, screen_height) = graphics::drawable_size(ctx);
        let paused = false;
        let slider_value = 50.0;
        let container = ParticleContainer::new(Vector4 {
            x: 0.0,
            y: 0.0,
            z: screen_height,
            w: screen_width,
        });
        let mut particles = Vec::new();
        // Create 1000 particles.
        for _ in 0..num {
            particles.push(Particle::new(&container));
        }
        Ok(MainState { particles, container, law, slider_value, paused})
    }

    pub fn pause_play(&mut self) {
        self.paused = !self.paused ;
    }

    pub fn update_state(&mut self, ctx: &mut Context) -> GameResult<()>  {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        let new_dt = dt / 20.0;
        let mut i = 0;
        while i < 20 {
            self.compute_single_interaction(new_dt);
            i = i + 1;
        }
        Ok(())
    }

    pub fn average_kinetic_energy(&self) -> f32 {
        let tot: f32 = self.particles.iter().len() as f32;
        let mut tot_ke =  0.0;
        for particle in &self.particles {
            tot_ke += particle.kinetic_energy();
        }
        return tot_ke / tot
    }

    pub fn compute_single_interaction(&mut self, dt: f32) {
        let law_type: InteractionLawType = self.law;
        let law: Box<dyn InteractionLaw> = build_interaction_law(law_type);

        for particle in &mut self.particles {
            particle.reset_force()
        }

        // Check and resolve collisions between particles.
        let len = self.particles.len();
        for i in 0..len {
            // Use split_at_mut to get two mutable references without double-borrowing.
            for j in (i + 1)..len {
                let (left, right) = self.particles.split_at_mut(j);
                let particle_i: &mut Particle = &mut left[i];
                let particle_j: &mut Particle = &mut right[0];
                law.resolve(particle_i, particle_j);
            }
        }

        // Update each particle and check physics boundaries.
        for particle in &mut self.particles {
            particle.update(dt);
            self.container.collision(particle);
        }
    }
}
