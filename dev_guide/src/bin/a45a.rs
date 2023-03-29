// Struct update syntax

struct Particle {
    color: (u8, u8, u8),
    alpha: u8,
    size: (u32, u32),
    position: (i32, i32),
    velocity: i32,
    direction: f32,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            color: (255, 0, 255),
            alpha: 255,
            size: (100, 100),
            position: (0, 0),
            velocity: 0,
            direction: 0.0,
        }
    }
}

fn main() {
    // This will set the aplha to 127 and the
    // rest will be the default values.
    let particle = Particle {
        alpha: 127,
        ..Default::default()
    };

    let red_particle = Particle {
        color: (255, 0, 0),
        ..Default::default()
    };

    // this will take the velocity of 10, red color
    // and the rest values will be the default
    let fast_particle = Particle {
        velocity: 10,
        ..red_particle
    };
}
