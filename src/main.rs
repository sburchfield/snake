use ggez::event;
use ggez::ContextBuilder;

mod game;
use game::SnakeGame;

fn main() -> ggez::GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("snake_game", "your_name")
    .build()?;
    let game = SnakeGame::new(&mut ctx);
    event::run(ctx, event_loop, game);
}