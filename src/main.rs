use ::rand::random;
use macroquad::color::hsl_to_rgb;
use macroquad::prelude::*;

const WINDOW_WIDTH: f32 = 800.;
const WIDOW_HEIGHT: f32 = 800.;

const GRID_WIDTH: usize = 100;

type Hue = f32;

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
    // 2d array of hue values
    let mut current_color = 0.; // 0. - 1.
    let mut canvas: Vec<Vec<Hue>> = vec![vec![current_color; GRID_WIDTH]; GRID_WIDTH]; // 2d array of hue values (0. is black)
    let mut radius = 1;

    loop {
        // Shift the color
        current_color = if current_color < 1. {
            current_color + 0.001
        } else {
            0.001
        };

        // Check if mouse is pressed
        if is_mouse_button_down(MouseButton::Left) {
            // get mouse position
            let mouse_position = mouse_position();

            // get the grid position
            let grid_x = mouse_position.0 / WINDOW_WIDTH * GRID_WIDTH as f32;
            let grid_y = mouse_position.1 / WIDOW_HEIGHT * GRID_WIDTH as f32;

            // check if out of bounds
            if !(grid_x < GRID_WIDTH as f32
                && grid_y < GRID_WIDTH as f32
                && grid_x > 0.
                && grid_y > 0.)
            {
                continue;
            }

            // convert to usize (its safe)
            let grid_x = grid_x as usize;
            let grid_y = grid_y as usize;

            // set the cell
            canvas[grid_x as usize][grid_y as usize] = current_color;

            // spawn the sand (50% probability, around the mouse position based on the radius)
            for x in grid_x - radius..grid_x + radius {
                for y in grid_y - radius..grid_y + radius {
                    if x < GRID_WIDTH && y < GRID_WIDTH {
                        if random::<bool>() {
                            // check if its in the bounds and check if it's already placed
                            if x < GRID_WIDTH && y < GRID_WIDTH && canvas[x][y] == 0. {
                                // set the cell
                                canvas[x][y] = current_color;
                            }
                        }
                    }
                }
            }
        }

        if mouse_wheel().1 > 0. {
            radius += (mouse_wheel().1 as f32 / 120.) as usize;
        } else if mouse_wheel().1 < 0. {
            radius -= (mouse_wheel().1.abs() as f32 / 120.) as usize;
        }

        // check if R is pressed and reset the canvas
        if is_key_pressed(KeyCode::R) {
            canvas = vec![vec![0.; GRID_WIDTH]; GRID_WIDTH];
        }

        // update the grid
        let mut new_canvas = canvas.clone();
        // loop through the grid
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_WIDTH {
                // if the cell is true
                if canvas[x][y] > 0. {
                    // check if the cell below is false and if it is not the bottom row
                    if y < GRID_WIDTH - 1 && canvas[x][y + 1] == 0. {
                        // set the cell below
                        new_canvas[x][y + 1] = canvas[x][y];
                        // remove the current cell
                        new_canvas[x][y] = 0.;
                    }
                    // if the cell below is true
                    else if y < GRID_WIDTH - 1 && canvas[x][y + 1] > 0. {
                        // check randomly if the cell to the left-bottom or right-bottom is false
                        let random_bool = random::<bool>();
                        if random_bool && x > 0 && canvas[x - 1][y + 1] == 0. {
                            // set the cell to the left-bottom
                            new_canvas[x - 1][y + 1] = canvas[x][y];
                            // remove the current cell
                            new_canvas[x][y] = 0.;
                        } else if !random_bool && x < GRID_WIDTH - 1 && canvas[x + 1][y + 1] == 0. {
                            // set the cell to the right-bottom
                            new_canvas[x + 1][y + 1] = canvas[x][y];
                            // remove the current cell
                            new_canvas[x][y] = 0.;
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
                if cell > 0. {
                    draw_rectangle(
                        x as f32 / GRID_WIDTH as f32 * WINDOW_WIDTH,
                        y as f32 / GRID_WIDTH as f32 * WIDOW_HEIGHT,
                        WINDOW_WIDTH / GRID_WIDTH as f32,
                        WIDOW_HEIGHT / GRID_WIDTH as f32,
                        // convert the hue to rgb
                        hsl_to_rgb(cell, 0.5, 0.5),
                    );
                }
            }
        }

        // show the radius
        draw_text(&format!("Radius: {}", radius), 10., 30., 30., WHITE);

        next_frame().await
    }
}
