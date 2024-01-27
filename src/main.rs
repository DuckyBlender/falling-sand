// Disable terminal
#![windows_subsystem = "windows"]

use macroquad::color::hsl_to_rgb;
use macroquad::prelude::*;

const WINDOW_WIDTH: f32 = 800.;
const WIDOW_HEIGHT: f32 = 800.;

const GRID_WIDTH: usize = 350;

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

// xor-shift random number generator
fn random_bool() -> bool {
    static mut X: u32 = 123456789;
    static mut Y: u32 = 362436069;
    static mut Z: u32 = 521288629;
    static mut W: u32 = 88675123;

    unsafe {
        let t = X ^ (X << 11);
        X = Y;
        Y = Z;
        Z = W;
        W = W ^ (W >> 19) ^ (t ^ (t >> 8));
        W & 1 == 1
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // 2d array of hue values
    let mut current_color = 0.; // 0. - 1.
    let mut canvas: Vec<Vec<Hue>> = vec![vec![current_color; GRID_WIDTH]; GRID_WIDTH]; // 2d array of hue values (0. is black)
    let mut radius = 1 * GRID_WIDTH / 100; // reasonable default

    loop {
        // Shift the color
        current_color = if current_color < 1. {
            current_color + 0.001
        } else {
            0.001
        };

        // Check if mouse is in the window
        if mouse_position().0 > 0.
            && mouse_position().1 > 0.
            && mouse_position().0 < WINDOW_WIDTH
            && mouse_position().1 < WIDOW_HEIGHT
        {
            // Check if mouse is pressed
            if is_mouse_button_down(MouseButton::Left) {
                // get mouse position
                let mouse_position = mouse_position();

                // check if out of bounds
                if mouse_position.0 < 0.
                    || mouse_position.1 < 0.
                    || mouse_position.0 > WINDOW_WIDTH
                    || mouse_position.1 > WIDOW_HEIGHT
                {
                    continue;
                }

                // get the grid position
                let grid_x = (mouse_position.0 / WINDOW_WIDTH * GRID_WIDTH as f32) as isize;
                let grid_y = (mouse_position.1 / WIDOW_HEIGHT * GRID_WIDTH as f32) as isize;

                // spawn the sand (50% probability, around the mouse position based on the radius)
                // ignore the out of bounds
                for x in (grid_x - radius as isize)..(grid_x + radius as isize) {
                    for y in (grid_y - radius as isize)..(grid_y + radius as isize) {
                        // check if out of bounds
                        if x < 0
                            || y < 0
                            || x > (GRID_WIDTH - 1) as isize
                            || y > (GRID_WIDTH - 1) as isize
                        {
                            continue;
                        }

                        // check if the cell is empty
                        if canvas[x as usize][y as usize] == 0. {
                            // check if the cell is in the radius
                            if ((x - grid_x).pow(2) + (y - grid_y).pow(2))
                                < (radius.pow(2) as isize)
                            {
                                // spawn the sand with 50% probability
                                if random_bool() {
                                    canvas[x as usize][y as usize] = current_color;
                                }
                            }
                        }
                    }
                }
            }
        }

        if mouse_wheel().1 > 0. {
            radius += (mouse_wheel().1 as f32 / 120.) as usize;
        } else if mouse_wheel().1 < 0. {
            if radius > mouse_wheel().1.abs() as usize / 120 {
                radius -= (mouse_wheel().1.abs() as f32 / 120.) as usize;
            }
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
                        let random_bool = random_bool();
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
        draw_text(
            &format!("FPS: {}", get_fps()),
            10.,
            WIDOW_HEIGHT - 80.,
            30.,
            WHITE,
        );
        draw_text("Press R to reset", 10., WIDOW_HEIGHT - 40., 30., WHITE);

        next_frame().await
    }
}
