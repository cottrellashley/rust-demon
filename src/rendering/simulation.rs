use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color, DrawMode, MeshBuilder, Text};
use ggez::mint::{Point2, Vector4};
use ggez::{Context, GameResult};
use rand::Rng;
use crate::physics::container::ParticleContainer;
use crate::physics::particles::Particle;

// Main state holding the collection of particles.
struct MainState {
    particles: Vec<Particle>,
    container: ParticleContainer,
    demon: bool,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let (screen_width, screen_height) = graphics::drawable_size(ctx);
        let container = ParticleContainer::new(Vector4 {
            x: 0.0,
            y: 0.0,
            z: screen_height,
            w: screen_width,
        });
        let mut particles = Vec::new();
        // Create 1000 particles.
        for _ in 0..1000 {
            particles.push(Particle::new(&container));
        }
        Ok(MainState { particles, container, demon: true})
    }

    fn average_kinetic_energy(&self) -> f32 {
        let tot: f32 = self.particles.iter().len() as f32;
        let mut tot_ke =  0.0;
        for particle in &self.particles {
            tot_ke += particle.kinetic_energy();
        }
        return tot_ke / tot
    }
}

impl EventHandler for MainState {
    // Update the simulation.
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        // Update each particle and check physics boundaries.
        for particle in &mut self.particles {
            particle.update(dt);
            self.container.collision(particle, self.demon);
        }

        // Check and resolve collisions between particles.
        let len = self.particles.len();
        for i in 0..len {
            // Use split_at_mut to get two mutable references without double-borrowing.
            for j in (i + 1)..len {
                let (left, right) = self.particles.split_at_mut(j);
                let particle_i = &mut left[i];
                let particle_j = &mut right[0];
                particle_i.detect_particle_collision(particle_j);
            }
        }
        Ok(())
    }

    // Draw the particles, the boundary box, and the realtime temperature readings.
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Clear the screen with a white background.

        // Blue: (0, 0, 255) Bottom 20%
        // Red: (255, 0, 0) Top 20%
        // Yellow: (255, 255, 0) Bottom Middle
        // Orange: (255, 165, 0)
        // Color from Kinetic Energy
        // Blue ->
        graphics::clear(ctx, Color::from_rgb(255, 255, 255));

        let mut mb = MeshBuilder::new();

        // Draw the box boundary.
        let rect = graphics::Rect::new(
            self.container.boundaries.x,
            self.container.boundaries.y,
            self.container.boundaries.w,
            self.container.boundaries.z,
        );
        mb.rectangle(DrawMode::stroke(2.0), rect, Color::BLACK);

        let avg = self.average_kinetic_energy();
        // Draw each particle as a blue circle.
        for particle in &self.particles {
            mb.circle(
                DrawMode::fill(),
                particle.position,
                particle.radius,
                0.1,
                particle.color(&avg),
            );
        }

        let mesh = mb.build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

        // --- Added Feature: Calculate and display temperature ---

        // Compute the middle x position.
        let middle_x = self.container.boundaries.w / 2.0;

        let mut left_sum = 0.0;
        let mut left_count = 0;
        let mut right_sum = 0.0;
        let mut right_count = 0;

        // Compute kinetic energy per particle (assuming unit mass, KE = 0.5*v^2).
        for particle in &self.particles {
            let ke = 0.5 * (particle.velocity.x.powi(2) + particle.velocity.y.powi(2));
            if particle.position.x < middle_x {
                left_sum += ke;
                left_count += 1;
            } else {
                right_sum += ke;
                right_count += 1;
            }
        }
        let left_temp = if left_count > 0 {
            left_sum / left_count as f32
        } else {
            0.0
        };
        let right_temp = if right_count > 0 {
            right_sum / right_count as f32
        } else {
            0.0
        };

        // Create text objects for the temperatures.
        let left_text = Text::new(format!("T_left: {:.2}", left_temp));
        let right_text = Text::new(format!("T_right: {:.2}", right_temp));

        // Position for left temperature (top left corner).
        let dest_left = Point2 { x: 5.0, y: 5.0 };

        // Position for right temperature (top right corner).
        let dest_right = Point2 {
            x: self.container.boundaries.w - 150.0,
            y: 5.0,
        };

        // Draw the temperature texts.
        graphics::draw(ctx, &left_text, (dest_left, Color::BLACK))?;
        graphics::draw(ctx, &right_text, (dest_right, Color::BLACK))?;

        graphics::present(ctx)?;
        Ok(())
    }

    // Activate or deactivate the textbox when the mouse is clicked.
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            self.demon = false;
        }
        else if button == MouseButton::Right {
            self.demon = true;
        }
    }
}

pub fn run_sim() -> GameResult {
    let cb = ggez::ContextBuilder::new("particle_simulation", "YourName")
        .window_setup(ggez::conf::WindowSetup::default().title("Particle Simulation"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1600.0, 1200.0));
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
