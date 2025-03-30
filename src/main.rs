use ggez::GameResult;
use crate::physics::laws::InteractionLawType;
use crate::rendering::simulation;

mod physics;
mod rendering;

pub fn main() -> GameResult {
    simulation::run(500, InteractionLawType::ImpulseCollision)
}
