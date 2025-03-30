use ggez::mint::Vector2;
use rand::Rng;

// Structure representing a single particle.
pub fn mod_f32(x: f32) -> f32 {
    if x < 0.0 {
        return -x
    } else {
        return x
    }
}

pub fn random_vector(max: f32, min: f32) -> Vector2<f32> {
    let mut rng = rand::thread_rng();
    let speed = rng.gen_range(min..max);
    let angle = rng.gen_range(0.0..(2.0 * std::f32::consts::PI));
    let vx = angle.cos() * speed;
    let vy = angle.sin() * speed;
    return Vector2 { x: vx, y: vy };
}
