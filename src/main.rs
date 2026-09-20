use macroquad::prelude::*;

enum State {
    Landing,
    Run,
    GameOver,
    Win,
}

#[macroquad::main("BasicShapes")]
async fn main() {
    // Game State
    let mut state: State = State::Landing;

    // Ball vars
    let mut ball_x: f32 = 0.0;
    let speed_x: f32 = 5.0;
    let mut go_x: bool = true;

    let mut ball_y: f32 = 0.0;
    let speed_y: f32 = 10.0;
    let mut go_y: bool = false;

    let ball_radius: f32 = 20.0;

    // Platform var
    let mut platform_offset: f32 = 0.0;
    let platform_speed: f32 = 10.0;
    let platform_width: f32 = 200.0;
    let platform_height: f32 = 10.0;
    let platform_y: f32 = 300.0;

    // Bricks 
    let mut grid: [[bool; 4]; 3] = [[true; 4]; 3];
    let brick_width: f32 = 180.0;
    let brick_height: f32 = 50.0;

    loop {
        // Bound
        draw_rectangle_lines(
            screen_width() / 2.0 - 500.0,
            screen_height() / 2.0 - 500.0,
            1000.0,
            1000.0,
            10.0,
            WHITE,
        );

        match &state {
            State::Landing => {
                draw_text("press SPACE to start", screen_width() / 2.0 - 220.0, screen_height() / 2.0, 50.0, WHITE);
                draw_text("How to play: use Left and Right arrow keys to move the platform", screen_width() / 2.0 - 330.0, screen_height() / 2.0 + 40.0, 24.0, WHITE);
                if is_key_pressed(KeyCode::Space) {
                    state = State::Run;
                }
            },
            State::Run => {
                if grid.iter().all(|row| row.iter().all(|&cell| cell == false)) {
                    state = State::Win;
                }

                // Platform
                if is_key_down(KeyCode::Right) && platform_offset <= 400.0 {
                    platform_offset += platform_speed;
                    if platform_offset > 400.0 {
                        platform_offset -= platform_offset - 400.0
                    };
                }
                if is_key_down(KeyCode::Left) && platform_offset >= -400.0 {
                    platform_offset -= platform_speed;
                    if platform_offset < -400.0 {
                        platform_offset += -400.0 - platform_offset
                    }
                }

                draw_rectangle(
                    screen_width() / 2.0 - platform_width / 2.0 + platform_offset,
                    screen_height() / 2.0 + platform_y,
                    platform_width,
                    platform_height,
                    WHITE,
                );

                // Bouncing Circle
                let platform_left: f32 = platform_offset - platform_width / 2.0;
                let platform_right: f32 = platform_offset + platform_width / 2.0;
                let ball_overlaps_platform: bool = ball_x + ball_radius >= platform_left && ball_x - ball_radius <= platform_right;
                let ball_screen_x: f32 = screen_width() / 2.0 + ball_x;
                let ball_screen_y: f32 = screen_height() / 2.0 + ball_y;

                if go_y && ball_y + ball_radius >= platform_y && ball_overlaps_platform {
                    ball_y = platform_y - ball_radius;
                    go_y = false;
                }

                ball_x = if go_x {
                    ball_x + speed_x
                } else {
                    ball_x - speed_x
                };
                ball_y = if go_y {
                    ball_y + speed_y
                } else {
                    ball_y - speed_y
                };

                if ball_x >= 480.0 {
                    ball_x -= ball_x - 480.0;
                    go_x = false;
                } else if ball_x <= -480.0 {
                    ball_x += -480.0 - ball_x;
                    go_x = true;
                }

                if ball_y >= 480.0 {
                    state = State::GameOver;
                } else if ball_y <= -480.0 {
                    ball_y += -480.0 - ball_y;
                    go_y = true;
                }

                draw_circle(
                    ball_screen_x,
                    ball_screen_y,
                    ball_radius,
                    WHITE,
                );

                // Bricks
                let mut grid_y = screen_height() / 2.0 - 400.0;

                for row in grid.iter_mut() {
                    let mut grid_x = screen_width() / 2.0 - 400.0;

                    for brick in row.iter_mut() {
                        if *brick {
                            let hit =
                                ball_screen_x + ball_radius >= grid_x
                                && ball_screen_x - ball_radius <= grid_x + brick_width
                                && ball_screen_y + ball_radius >= grid_y
                                && ball_screen_y - ball_radius <= grid_y + brick_height;

                            if hit {
                                *brick = false;
                                go_y = !go_y;
                            }

                            draw_rectangle(grid_x, grid_y, brick_width, brick_height, WHITE);
                        }
                        grid_x += 200.0;
                    }
                    grid_y += 70.0;
                }
            },
            State::GameOver => {
                ball_x = 0.0;
                ball_y = 0.0;
                go_x = true;
                go_y = false;
                platform_offset = 0.0;
                grid = [[true; 4]; 3];

                draw_text("GAME OVER", screen_width() / 2.0 - 200.0, screen_height() / 2.0, 100.0, RED);
                draw_text("press SPACE to replay", screen_width() / 2.0 - 100.0, screen_height() / 2.0 + 40.0, 24.0, WHITE);
                if is_key_pressed(KeyCode::Space) {
                    state = State::Run;
                }
            },
            State::Win => {
                ball_x = 0.0;
                ball_y = 0.0;
                go_x = true;
                go_y = false;
                platform_offset = 0.0;
                grid = [[true; 4]; 3];

                draw_text("CLEARED", screen_width() / 2.0 - 150.0, screen_height() / 2.0, 100.0, GREEN);
                draw_text("press SPACE to replay", screen_width() / 2.0 - 100.0, screen_height() / 2.0 + 40.0, 24.0, WHITE);
                if is_key_pressed(KeyCode::Space) {
                    state = State::Run;
                }
            }
        }

        next_frame().await
    }

}
