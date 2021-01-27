use ggez;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::event;

struct MainState {
    
}

impl MainState {
    pub fn new() -> Self {
        MainState {}
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Pong_0", "Vic");
    let (ctx, event_loop) = &mut cb.build()?;
    graphics::set_window_title(ctx, "PONG");

    let mut state = MainState::new();

    event::run(ctx, event_loop, &mut state);
    Ok(())
}
