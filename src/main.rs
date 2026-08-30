use macroquad::prelude::*;

const BLOCK_SIZE: i32 = 8;
const GRID_COLUMNS: i32 = 160;
const GRID_ROWS: i32 = 104;

#[macroquad::main(get_game_conf())]
async fn main() {
    loop {
        clear_background(RED);

        for i in 0..=GRID_COLUMNS {
            draw_line(
                BLOCK_SIZE as f32 * i as f32,
                0.,
                BLOCK_SIZE as f32 * i as f32,
                BLOCK_SIZE as f32 * GRID_ROWS as f32,
                2.0,
                GREEN,
            );
        }
        
        for i in 0..=GRID_ROWS {
            draw_line(
                0.,
                BLOCK_SIZE as f32 * i as f32,
                BLOCK_SIZE as f32 * GRID_COLUMNS as f32,
                BLOCK_SIZE as f32 * i as f32,
                2.0,
                GREEN,
            );
        }

        next_frame().await;
    }
}

fn get_game_conf() -> Conf {
    Conf {
        window_title: String::from("Sand game"),
        window_width: BLOCK_SIZE * GRID_COLUMNS,
        window_height: BLOCK_SIZE * GRID_ROWS,
        window_resizable:false,
        ..Default::default()
    }
}

struct Sand {
    x: f32,
    y: f32,

    dx: f32,
    dy: f32,
}

impl Sand {
    pub fn new(x:f32, y:f32, dx:f32, dy:f32)->Self{
        Self{x,y,dx,dy}
    }

    pub fn paint(self) {
        draw_rectangle(
            self.x, 
            self.y,
            BLOCK_SIZE as f32,
            BLOCK_SIZE as f32,
            ORANGE,
        )
    }

    pub fn move_sand(&mut self){
    } 
}
