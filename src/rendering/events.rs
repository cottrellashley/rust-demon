use ggez::{graphics, Context, GameResult};
use ggez::event::{EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, MeshBuilder, Text};
use ggez::mint::Point2;
use crate::rendering::state::MainState;


fn draw_slider(
    ctx: &mut Context,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    current_value: f32,
) -> GameResult<()> {
    // Draw slider track.
    let track_rect = graphics::Rect::new(x, y, width, height);
    let track_color = Color::from_rgb(100, 100, 100);
    let track = Mesh::new_rectangle(ctx, DrawMode::fill(), track_rect, track_color)?;
    graphics::draw(ctx, &track, DrawParam::default())?;

    // Compute the thumb position.
    let thumb_width = 10.0;
    let thumb_height = height + 4.0;
    let thumb_x = x + (current_value / 100.0) * width - thumb_width / 2.0;
    let thumb_y = y - 2.0;
    let thumb_rect = graphics::Rect::new(thumb_x, thumb_y, thumb_width, thumb_height);
    let thumb_color = Color::from_rgb(200, 200, 200);
    let thumb = Mesh::new_rectangle(ctx, DrawMode::fill(), thumb_rect, thumb_color)?;
    graphics::draw(ctx, &thumb, DrawParam::default())?;

    Ok(())
}


impl EventHandler for MainState {
    // Update function remains unchanged.
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.paused {
            return Ok(());
        }
        self.update_state(ctx)
    }

    // Draw the simulation inside a box, leaving a sidebar on the right.
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Clear the screen with a background color.
        graphics::clear(ctx, Color::from_rgb(7, 18, 50));

        // Define the sidebar width.
        let sidebar_width = 150.0;
        // The simulation region is the total width minus the sidebar.
        let sim_width = self.container.boundaries.w - sidebar_width;

        // Define the simulation region rectangle.
        let simulation_rect = graphics::Rect::new(
            self.container.boundaries.x,
            self.container.boundaries.y,
            sim_width,
            self.container.boundaries.z,
        );

        let mut mb = MeshBuilder::new();
        // Draw a white boundary around the simulation region.
        mb.rectangle(DrawMode::stroke(2.0), simulation_rect, Color::WHITE);

        // Compute the average kinetic energy for particle coloring.
        let avg = self.average_kinetic_energy();

        // Draw each particle as a circle.
        for particle in &self.particles {
            mb.circle(
                DrawMode::fill(),
                particle.position,
                particle.radius,
                0.1,
                particle.color(&avg),
            );
        }

        let sim_mesh = mb.build(ctx)?;
        graphics::draw(ctx, &sim_mesh, graphics::DrawParam::default())?;

        // --- Sidebar UI Elements ---
        // Compute temperature readings as before.
        let middle_x = simulation_rect.w / 2.0;
        let mut left_sum = 0.0;
        let mut left_count = 0;
        let mut right_sum = 0.0;
        let mut right_count = 0;

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
        let left_temp = if left_count > 0 { left_sum / left_count as f32 } else { 0.0 };
        let right_temp = if right_count > 0 { right_sum / right_count as f32 } else { 0.0 };

        // Define the sidebar region (on the right side of the screen).
        let sidebar_rect = graphics::Rect::new(
            sim_width, // starting x coordinate of the sidebar
            0.0,
            sidebar_width,
            self.container.boundaries.z,
        );

        // Draw a background for the sidebar.
        let sidebar_bg = MeshBuilder::new()
            .rectangle(DrawMode::fill(), sidebar_rect, Color::from_rgb(50, 50, 50))?
            .build(ctx)?;
        graphics::draw(ctx, &sidebar_bg, graphics::DrawParam::default())?;

        // Create text objects to display temperature readings.
        let left_text = Text::new(format!("T_left: {:.2}", left_temp));
        let right_text = Text::new(format!("T_right: {:.2}", right_temp));

        // Position the text within the sidebar.
        let text_dest1 = Point2 { x: sim_width + 10.0, y: 10.0 };
        let text_dest2 = Point2 { x: sim_width + 10.0, y: 40.0 };

        graphics::draw(ctx, &left_text, (text_dest1, Color::WHITE))?;
        graphics::draw(ctx, &right_text, (text_dest2, Color::WHITE))?;

        // Draw the slider.
        // Define slider dimensions and position.
        let slider_x = sim_width + 10.0;
        let slider_y = 80.0;
        let slider_width = sidebar_width - 20.0;
        let slider_height = 20.0;
        draw_slider(ctx, slider_x, slider_y, slider_width, slider_height, self.slider_value)?;

        // Optionally, display the current slider value.
        let slider_val_text = Text::new(format!("Value: {:.0}", self.slider_value));
        let slider_val_dest = Point2 { x: slider_x, y: slider_y + slider_height + 5.0 };
        graphics::draw(ctx, &slider_val_text, (slider_val_dest, Color::WHITE))?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        // Check if the click is inside the slider area.
        let sidebar_width = 150.0;
        let sim_width = self.container.boundaries.w - sidebar_width;
        let slider_x = sim_width + 10.0;
        let slider_y = 80.0;
        let slider_width = sidebar_width - 20.0;
        let slider_height = 20.0;
        let slider_rect = graphics::Rect::new(slider_x, slider_y, slider_width, slider_height);

        if slider_rect.contains([x, y]) {
            // Update slider value based on the x position.
            // Map x from slider_x..(slider_x+slider_width) to 0.0..100.0.
            self.slider_value = ((x - slider_x) / slider_width * 100.0).clamp(0.0, 100.0);
        } else {
            // Other mouse button events.
            if button == MouseButton::Left {
                self.container.demon_looking = false;
            } else if button == MouseButton::Right {
                self.container.demon_looking = true;
            }
        }
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Space => { self.pause_play(); },
            KeyCode::Right => { self.update_state(_ctx); },
            _ => {}
        }
    }
}
