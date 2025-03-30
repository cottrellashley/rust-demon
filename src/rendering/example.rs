use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, Color, DrawMode, MeshBuilder, Rect, Text};
use ggez::mint::Point2;
use ggez::{Context, ContextBuilder, GameResult};

struct MainState {
    // The area of our textbox.
    textbox_rect: Rect,
    // The content that the user types.
    textbox_content: String,
    // Whether the textbox is currently active (focused).
    textbox_active: bool,
    // Logger.
    log: String,
}

impl MainState {
    fn new() -> Self {
        // Define a rectangle for the textbox at position (50, 50) with width 300 and height 50.
        let textbox_rect = Rect::new(50.0, 50.0, 300.0, 50.0);
        MainState {
            textbox_rect,
            textbox_content: String::new(),
            textbox_active: false,
            log: String::from("Log box..."),
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Clear the screen with a white background.
        graphics::clear(ctx, Color::from_rgb(255, 255, 255));

        // Draw the textbox border.
        let textbox_border = MeshBuilder::new()
            .rectangle(DrawMode::stroke(2.0), self.textbox_rect, Color::BLACK)?
            .build(ctx)?;
        graphics::draw(ctx, &textbox_border, graphics::DrawParam::default())?;

        // Draw the text inside the textbox.
        let text = Text::new(self.textbox_content.clone());
        graphics::draw(
            ctx,
            &text,
            (Point2 {
                x: self.textbox_rect.x + 5.0,
                y: self.textbox_rect.y + 5.0,
            }, Color::BLACK),
        )?;

        graphics::present(ctx)?;
        Ok(())
    }

    // Activate or deactivate the textbox when the mouse is clicked.
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            if self.textbox_rect.contains(Point2 { x, y }) {
                self.textbox_active = true;
            } else {
                self.textbox_active = false;
            }
        }
    }

    // Handle special keys like backspace and return.
    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        if self.textbox_active {
            match keycode {
                KeyCode::Back => { self.textbox_content.pop(); },
                KeyCode::Return => { self.textbox_content.push('\n'); },
                _ => {}
            }
        }
    }

    // Capture text input events.
    fn text_input_event(&mut self, _ctx: &mut Context, ch: char) {
        if self.textbox_active {
            self.textbox_content.push(ch);
        }
    }
}

pub fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("textbox_example", "YourName")
        .window_setup(ggez::conf::WindowSetup::default().title("Textbox Example"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()?;
    let state = MainState::new();
    event::run(ctx, event_loop, state)
}
