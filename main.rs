use macroquad::prelude::*;
use macroquad::rand;

#[derive(PartialEq, Copy, Clone)]
enum Dir { Up, Down, Left, Right }

struct Point {
    x: i32,
    y: i32,
}

struct Game {
    snake: Vec<Point>,
    dir:   Dir,
    nextdir: Dir,
    food:  Point,
    score: i32,
}

#[macroquad::main("Snake")]
async fn main() {
    let mut game = Game {
        snake: vec![
            Point { x: 5, y: 10 },
            Point { x: 4, y: 10 },
            Point { x: 3, y: 10 },
        ],
        dir: Dir::Right,
        nextdir: Dir::Right,
        food: Point { x: 10, y: 10 },
        score: 0,
    };

    let mut last_move = get_time();

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Up)    && game.dir != Dir::Down  { game.nextdir = Dir::Up;    }
        if is_key_pressed(KeyCode::Down)  && game.dir != Dir::Up    { game.nextdir = Dir::Down;  }
        if is_key_pressed(KeyCode::Left)  && game.dir != Dir::Right { game.nextdir = Dir::Left;  }
        if is_key_pressed(KeyCode::Right) && game.dir != Dir::Left  { game.nextdir = Dir::Right; }

        if get_time() - last_move > 0.075 {
            last_move = get_time();
            game.dir = game.nextdir;

            let new_head = match game.dir {
                Dir::Right => Point { x: game.snake[0].x + 1, y: game.snake[0].y     },
                Dir::Left  => Point { x: game.snake[0].x - 1, y: game.snake[0].y     },
                Dir::Up    => Point { x: game.snake[0].x,     y: game.snake[0].y - 1 },
                Dir::Down  => Point { x: game.snake[0].x,     y: game.snake[0].y + 1 },
            };
            game.snake.insert(0, new_head);

            if game.snake[0].x == game.food.x && game.snake[0].y == game.food.y {
                game.score += 1;
                game.food = Point {
                    x: rand::gen_range(0, 20),
                    y: rand::gen_range(0, 20),
                };
            } else {
                game.snake.pop();
            }
            
            for seg in &game.snake[1..] {
                if game.snake[0].x == seg.x && game.snake[0].y == seg.y {
                    println!("game over! score: {}", game.score);
                    game = Game {
                        snake: vec![
                            Point { x: 5, y: 10 },
                            Point { x: 4, y: 10 },
                            Point { x: 3, y: 10 },
                        ],
                        dir: Dir::Right,
                        nextdir: Dir::Right,
                        food: Point { x: 10, y: 10 },
                        score: 0,
                    };
                    break;
                }
            }

            if game.snake[0].x < 0 || game.snake[0].x >= 20
            || game.snake[0].y < 0 || game.snake[0].y >= 20 {
                println!("game over! score: {}", game.score);
                game = Game {
                    snake: vec![
                        Point { x: 5, y: 10 },
                        Point { x: 4, y: 10 },
                        Point { x: 3, y: 10 },
                    ],
                    dir: Dir::Right,
                    nextdir: Dir::Right,
                    food: Point { x: 10, y: 10 },
                    score: 0,
                };
            }
        }

        let cell = 30.0;
        for seg in &game.snake {
            draw_rectangle(seg.x as f32 * cell, seg.y as f32 * cell, cell, cell, GREEN);
        }
        draw_rectangle(game.food.x as f32 * cell, game.food.y as f32 * cell, cell, cell, RED);
        draw_rectangle_lines(0.0, 0.0, 20.0 * cell, 20.0 * cell, 2.0, WHITE);
        next_frame().await;

    } // loop

} // main