use ggez::{graphics, Context, GameResult};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{Color, DrawMode, MeshBuilder, Text};
use ggez::mint::Point2;
use crate::rendering::state::MainState;

impl EventHandler for MainState {
    // Update the simulation.
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        self.compute_single_interaction(dt);
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
            self.container.demon_looking = false;
        }
        else if button == MouseButton::Right {
            self.container.demon_looking = true;
        }
    }
}
