use crate::engine::global_const::{BLOCK_SIZE, GRID_COLUMNS, GRID_ROWS};
use macroquad::color::Color;
use macroquad::prelude::*;

#[derive(Debug)]
pub enum SidewaysDirection {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Sand {
    pub x: f32,
    pub y: f32,
    pub direction: Option<SidewaysDirection>,
    pub sand_color: Color,

    dx: f32,
    dy: f32,
}

impl Sand {
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        Self {
            x,
            y,
            sand_color: color,
            dx: BLOCK_SIZE as f32,
            dy: BLOCK_SIZE as f32,
            direction: None,
        }
    }

    pub fn paint(&self) {
        draw_rectangle(
            self.x,
            self.y,
            BLOCK_SIZE as f32,
            BLOCK_SIZE as f32,
            self.sand_color,
        );
    }

    pub fn move_particle_down(&mut self) {
        let is_in_bound = (self.y + BLOCK_SIZE as f32) < (BLOCK_SIZE as f32 * GRID_ROWS as f32);
        if is_in_bound {
            self.y += self.dy;
        }
    }

    pub fn allow_sideways_movement(&mut self) {
        match self.direction {
            None => {
                let direction = if rand::gen_range(0, 2) == 0 {
                    SidewaysDirection::Left
                } else {
                    SidewaysDirection::Right
                };

                self.direction = Some(direction)
            }
            Some(_) => (),
        }
    }

    pub fn move_particle_side_down(&mut self) {
        let is_in_bound =
            0. < self.x && (self.x + BLOCK_SIZE as f32) < (BLOCK_SIZE as f32 * GRID_COLUMNS as f32);
        if !is_in_bound {
            return;
        }

        match &mut self.direction {
            Some(direction_val) => {
                // Side step
                match direction_val {
                    SidewaysDirection::Left => self.x -= self.dx,
                    SidewaysDirection::Right => self.x += self.dx,
                }
            }
            None => (),
        }
    }
}
