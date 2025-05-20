use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawMode, Rect};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::event::EventHandler;
use rand::Rng;

pub struct SnakeGame {
    pub snake: Vec<(i32, i32)>,
    pub apple: (i32, i32),
    pub dir: (i32, i32),
    pub grid_size: i32,
    pub grid_count_x: i32,   // ✅ horizontal tiles
    pub grid_count_y: i32,   // ✅ vertical tiles
    pub score: i32,
    pub game_over: bool,
    pub move_timer: f32,
    pub move_interval: f32,
}

impl SnakeGame {
    pub fn new(ctx: &mut Context) -> SnakeGame {

        let (screen_w, screen_h) = ctx.gfx.drawable_size();
        let grid_size = 20;
        let grid_count_x = (screen_w / grid_size as f32) as i32;
        let grid_count_y = (screen_h / grid_size as f32) as i32;

        SnakeGame {
            snake: vec![(10, 10)],
            apple: (15, 15),
            dir: (0, 0),
            grid_size,
            grid_count_x,
            grid_count_y,
            score: 0,
            game_over: false,
            move_timer: 0.0,
            move_interval: 0.075,  // Move every 0.2 seconds (5 moves per second)
        }
    }

    fn move_snake(&mut self) {
        let (dx, dy) = self.dir;
        if dx != 0 || dy != 0 {
            let (head_x, head_y) = self.snake.last().unwrap();
            let new_head = (
                (head_x + dx).rem_euclid(self.grid_count_x),
                (head_y + dy).rem_euclid(self.grid_count_y),
            );
    
            if self.snake.contains(&new_head) {
                self.game_over = true;
                return;
            }
    
            self.snake.push(new_head);
    
            if new_head != self.apple {
                self.snake.remove(0);
            } else {
                self.score += 1;
                let mut rng = rand::rng();
                self.apple = (
                    rng.random_range(0..self.grid_count_x), 
                    rng.random_range(1..self.grid_count_y),
                );
            }
        }
    }

    pub fn reset(&mut self) {
        self.snake = vec![(10, 10)];
        self.apple = (15, 15);
        self.dir = (0, 0);
        self.score = 0;
        self.game_over = false;
        self.move_timer = 0.0;
    }
    
}

impl EventHandler for SnakeGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.game_over {
            return Ok(());
        }
    
        // Time delta since last frame
        let dt = ctx.time.delta().as_secs_f32();
        self.move_timer += dt;
    
        // Only move if enough time has passed
        if self.move_timer >= self.move_interval {
            self.move_snake();
            self.move_timer = 0.0;
        }
    
        Ok(())
    }  

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
    
        // Draw snake
        for (x, y) in &self.snake {
            let rect = Rect::new(
                (*x as f32) * self.grid_size as f32,
                (*y as f32) * self.grid_size as f32,
                (self.grid_size - 2) as f32,
                (self.grid_size - 2) as f32,
            );
            let mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::GREEN)?;
            canvas.draw(&mesh, graphics::DrawParam::default());
        }
    
        // Draw apple
        let apple_rect = Rect::new(
            (self.apple.0 as f32) * self.grid_size as f32,
            (self.apple.1 as f32) * self.grid_size as f32,
            (self.grid_size - 2) as f32,
            (self.grid_size - 2) as f32,
        );
        let apple_mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), apple_rect, Color::RED)?;
        canvas.draw(&apple_mesh, graphics::DrawParam::default());

        if self.game_over {
            let text = graphics::Text::new(
                graphics::TextFragment::new("Game Over! Press R to Restart")
                    .scale(graphics::PxScale::from(24.0)),
            );            
            let screen_center = (self.grid_count_x as f32 * self.grid_size as f32) / 2.0;
        
            canvas.draw(
                &text,
                graphics::DrawParam::default()
                    .dest([screen_center - 195.0, 200.0]),
            );
        }

        let score_text = graphics::Text::new(
            graphics::TextFragment::new(format!("Score: {}", self.score))
                .scale(graphics::PxScale::from(20.0)),
        );
        canvas.draw(
            &score_text,
            graphics::DrawParam::default()
                .dest([10.0, 10.0]), // top-left corner
        );
    
        canvas.finish(ctx)?;
        Ok(())
    }
    

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeated: bool) -> GameResult {
        if let Some(keycode) = input.keycode {
            if self.game_over {
                if keycode == KeyCode::R {
                    self.reset();
                }
                return Ok(()); // Ignore all other keys if dead
            }
    
            match keycode {
                KeyCode::Left => self.dir = (-1, 0),
                KeyCode::Right => self.dir = (1, 0),
                KeyCode::Up => self.dir = (0, -1),
                KeyCode::Down => self.dir = (0, 1),
                _ => (),
            }
        }
        Ok(())
    }
}