use ::rand::random;
use macroquad::prelude::*;

const WINDOW_WIDTH: f32 = 800.;
const WIDOW_HEIGHT: f32 = 800.;

const GRID_WIDTH: usize = 100;

fn window_conf() -> Conf {
    Conf {
        window_title: "Falling sand!".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WIDTH as i32,
        window_height: WIDOW_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // 2d array of bools
    let mut canvas: Vec<Vec<bool>> = vec![vec![false; GRID_WIDTH]; GRID_WIDTH];

    loop {
        // check if mouse is pressed
        if is_mouse_button_down(MouseButton::Left) {
            // get mouse position
            let mouse_position = mouse_position();

            // get the grid position
            let grid_x = (mouse_position.0 / WINDOW_WIDTH * GRID_WIDTH as f32) as usize;
            let grid_y = (mouse_position.1 / WIDOW_HEIGHT * GRID_WIDTH as f32) as usize;

            // set the grid position to true
            canvas[grid_x][grid_y] = true;
        }

        // update the grid
        let mut new_canvas = canvas.clone();
        // loop through the grid
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_WIDTH {
                // if the cell is true
                if canvas[x][y] {
                    // check if the cell below is false and if it is not the bottom row
                    if y < GRID_WIDTH - 1 && !canvas[x][y + 1] {
                        // set the cell below to true
                        new_canvas[x][y + 1] = true;
                        // set the current cell to false
                        new_canvas[x][y] = false;
                    }
                    // if the cell below is true
                    else if y < GRID_WIDTH - 1 && canvas[x][y + 1] {
                        // check randomly if the cell to the left-bottom or right-bottom is false
                        let random_bool = random::<bool>();
                        if random_bool && x > 0 && !canvas[x - 1][y + 1] {
                            // set the cell to the left-bottom to true
                            new_canvas[x - 1][y + 1] = true;
                            // set the current cell to false
                            new_canvas[x][y] = false;
                        } else if !random_bool && x < GRID_WIDTH - 1 && !canvas[x + 1][y + 1] {
                            // set the cell to the right-bottom to true
                            new_canvas[x + 1][y + 1] = true;
                            // set the current cell to false
                            new_canvas[x][y] = false;
                        } else {
                            // do nothing
                        }
                    } else {
                        // do nothing
                    }
                }
            }
        }

        // set the canvas to the new canvas
        canvas = new_canvas;

        clear_background(BLACK);

        // draw the canvas
        for (x, row) in canvas.iter().enumerate() {
            for (y, &cell) in row.iter().enumerate() {
                if cell {
                    draw_rectangle(
                        x as f32 / GRID_WIDTH as f32 * WINDOW_WIDTH,
                        y as f32 / GRID_WIDTH as f32 * WIDOW_HEIGHT,
                        WINDOW_WIDTH / GRID_WIDTH as f32,
                        WIDOW_HEIGHT / GRID_WIDTH as f32,
                        WHITE,
                    );
                }
            }
        }

        next_frame().await
    }
}
