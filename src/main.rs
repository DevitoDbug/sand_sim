use macroquad::color::Color;
use macroquad::prelude::*;

use crate::engine::{
    game,
    global_const::{BLOCK_SIZE, GRID_COLUMNS, GRID_ROWS},
    sand::{Sand, SidewaysDirection},
};

mod engine;

#[macroquad::main(get_game_conf())]
async fn main() {
    let mut game = game::Game::new();
    let layers = vec![RED, ORANGE, GREEN, GOLD, SKYBLUE, PURPLE];
    let mut current_layer_index = 0;
    let mut layer_timer = 0.0;
    let layer_duration = 5.0;
    loop {
        clear_background(WHITE);

        for i in 0..game.sand.len() {
            game.sand[i].paint();

            if has_down_collision(&game.sand, i) {
                game.sand[i].allow_sideways_movement();
                if !has_side_down_collision(&game.sand, i) {
                    game.sand[i].move_particle_side_down();
                }
            } else {
                game.sand[i].move_particle_down();
            }
        }

        gen_sand(&mut game.sand, layers[current_layer_index]);
        layer_timer += get_frame_time();
        if layer_timer >= layer_duration {
            layer_timer = 0.;

            current_layer_index = (current_layer_index + 1) % layers.len();
        }

        next_frame().await;
    }
}

fn get_game_conf() -> Conf {
    Conf {
        window_title: String::from("Sand game"),
        window_width: BLOCK_SIZE * GRID_COLUMNS,
        window_height: BLOCK_SIZE * GRID_ROWS,
        window_resizable: false,
        ..Default::default()
    }
}

fn gen_sand(sand: &mut Vec<Sand>, sand_color: Color) {
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

                sand.push(Sand::new(sand_x as f32, sand_y as f32, sand_color));
            }
        }
    }
}

fn has_down_collision(particles: &[Sand], target_particle_index: usize) -> bool {
    let target_particle = &particles[target_particle_index];
    for particle in particles {
        if particle.x == target_particle.x {
            if target_particle.y + BLOCK_SIZE as f32 == particle.y {
                // Trigger the sideways movement
                return true;
            }
        }
    }
    return false;
}

fn has_side_down_collision(particles: &[Sand], target_particle_index: usize) -> bool {
    let target_particle = &particles[target_particle_index];

    match &target_particle.direction {
        Some(target_particle_dirrection) => {
            for particle in particles {
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

fn shift_hue(input_color: Color, hue_shitf: f32) -> Color {
    let (h, s, l) = macroquad::color::rgb_to_hsl(input_color);
    let new_h = (h + hue_shitf) % 1.0;

    macroquad::color::hsl_to_rgb(new_h, s, l)
}
