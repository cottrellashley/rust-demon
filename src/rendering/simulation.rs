use ggez::event::{self};
use ggez::{ContextBuilder, GameResult};
use ggez::conf::{WindowSetup, WindowMode};
use crate::physics::laws::InteractionLawType;
use crate::rendering::state::MainState;


pub fn run(num: u32, law: InteractionLawType) -> GameResult {
    let cb: ContextBuilder = ContextBuilder::new("Maxwell's Demon Sim", "Ashley Cottrell")
        .window_setup(WindowSetup::default().title("Maxwell's Demon Simulation"))
        .window_mode(WindowMode::default().dimensions(1600.0, 1200.0));
    let (mut ctx, event_loop) = cb.build()?;
    let state: MainState = MainState::new(&mut ctx, num, law)?;
    event::run(ctx, event_loop, state)
}
