use crate::engine::{
    global_const::{BLOCK_SIZE, GRID_COLUMNS, GRID_ROWS},
    sand::{Sand, SidewaysDirection},
};
use macroquad::prelude::*;

pub struct Game {
    pub sand: Vec<Sand>,
}

impl Game {
    pub fn new() -> Self {
        Self { sand: vec![] }
    }

    pub async fn render(&mut self) {
        let layers = vec![RED, ORANGE, GREEN, GOLD, SKYBLUE, PURPLE];
        let mut current_layer_index = 0;
        let mut layer_timer = 0.0;
        let layer_duration = 5.0;
        loop {
            clear_background(WHITE);
            for i in 0..self.sand.len() {
                self.sand[i].paint();

                if self.has_down_collision(i) {
                    self.sand[i].allow_sideways_movement();
                    if !self.has_side_down_collision(i) {
                        self.sand[i].move_particle_side_down();
                    }
                } else {
                    self.sand[i].move_particle_down();
                }
            }

            self.gen_sand(layers[current_layer_index]);
            layer_timer += get_frame_time();
            if layer_timer >= layer_duration {
                layer_timer = 0.;

                current_layer_index = (current_layer_index + 1) % layers.len();
            }

            next_frame().await;
        }
    }

    fn gen_sand(&mut self, sand_color: Color) {
        let mouse_in_bound = {
            let (x, y) = mouse_position();
            (x <= BLOCK_SIZE as f32 * GRID_COLUMNS as f32)
                && (y <= BLOCK_SIZE as f32 * GRID_ROWS as f32)
        };

        let mouse_down =
            { is_mouse_button_down(MouseButton::Left) || is_mouse_button_down(MouseButton::Right) };

        if mouse_in_bound && mouse_down {
            let (x, y) = mouse_position();
            let (x, y) = (
                x as i32 / BLOCK_SIZE * BLOCK_SIZE,
                y as i32 / BLOCK_SIZE * BLOCK_SIZE,
            );
            for dx in -2..=2 {
                for dy in -2..=2 {
                    let sand_x = x + dx * BLOCK_SIZE;
                    let sand_y = y + dy * BLOCK_SIZE;

                    self.sand
                        .push(Sand::new(sand_x as f32, sand_y as f32, sand_color));
                }
            }
        }
    }

    fn has_down_collision(&self, target_particle_index: usize) -> bool {
        let target_particle = &self.sand[target_particle_index];
        for particle in &self.sand {
            if particle.x == target_particle.x {
                if target_particle.y + BLOCK_SIZE as f32 == particle.y {
                    // Trigger the sideways movement
                    return true;
                }
            }
        }
        return false;
    }

    fn has_side_down_collision(&self, target_particle_index: usize) -> bool {
        let target_particle = &self.sand[target_particle_index];

        match &target_particle.direction {
            Some(target_particle_dirrection) => {
                for particle in &self.sand {
                    match target_particle_dirrection {
                        SidewaysDirection::Right => {
                            if particle.x == target_particle.x + BLOCK_SIZE as f32
                                && particle.y == target_particle.y + BLOCK_SIZE as f32
                            {
                                return true;
                            }
                        }
                        SidewaysDirection::Left => {
                            if particle.x + BLOCK_SIZE as f32 == target_particle.x
                                && particle.y == target_particle.y + BLOCK_SIZE as f32
                            {
                                return true;
                            }
                        }
                    };
                }

                false
            }
            None => false,
        }
    }
}
