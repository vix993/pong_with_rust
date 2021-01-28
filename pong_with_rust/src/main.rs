use ggez;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::event;
use ggez::input::keyboard::{self, KeyCode};
use rand::{self, thread_rng, Rng};

const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;
const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const PLAYER_SPEED: f32 = 600.0;
const BALL_SPEED: f32 = 600.0;

fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

fn move_racket(pos: &mut na::Point2<f32>, keycode: KeyCode, y_dir: f32, ctx: &mut Context) {
    let dt = ggez::timer::delta(ctx).as_secs_f32();
    let screen_height = graphics::drawable_size(ctx).1;

    if keyboard::is_key_pressed(ctx, keycode) {
        pos.y += y_dir * PLAYER_SPEED * dt;
    }

    clamp(&mut pos.y, RACKET_HEIGHT_HALF, screen_height-RACKET_HEIGHT_HALF);
}

fn randomize_vec(vec: &mut na::Vector2<f32>, x: f32, y: f32) {
    let mut rng = thread_rng();
    vec.x = match rng.gen_bool(0.5) {
        true => x,
        false => -x,
    };
    vec.y = match rng.gen_bool(0.5) {
        true => y,
        false => -y,
    };
}


struct MainState {
    player_one_pos: na::Point2<f32>,
    player_two_pos: na::Point2<f32>,
    ball_pos: na::Point2<f32>,
    ball_vel: na::Vector2<f32>,
    player_one_score: i32,
    player_two_score: i32,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (screen_width, screen_height) = graphics::drawable_size(ctx);
        let (screen_width_half, screen_height_half) = (screen_width * 0.5, screen_height * 0.5);

        let mut ball_vel = na::Vector2::new(0.0, 0.0);
        randomize_vec(&mut ball_vel, BALL_SPEED, BALL_SPEED);
        MainState {
            player_one_pos : na::Point2::new(RACKET_WIDTH_HALF, screen_height_half),
            player_two_pos : na::Point2::new(screen_width - RACKET_HEIGHT_HALF, screen_height_half),
            ball_pos : na::Point2::new(screen_width_half, screen_height_half),
            ball_vel,
            player_one_score : 0,
            player_two_score : 0,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        let (screen_width, screen_height) = graphics::drawable_size(ctx);

        move_racket(&mut self.player_one_pos, KeyCode::W, 1.0, ctx);
        move_racket(&mut self.player_one_pos, KeyCode::S, -1.0, ctx);
        move_racket(&mut self.player_two_pos, KeyCode::Up, 1.0, ctx);
        move_racket(&mut self.player_two_pos, KeyCode::Down, -1.0, ctx);

        self.ball_pos += self.ball_vel * dt;
        if self.ball_pos.x < 0.0 {
            self.ball_pos.x = screen_width * 0.5;
            self.ball_pos.y = screen_height * 0.5;
            randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
            self.player_two_score += 1;
        }
        if self.ball_pos.x < screen_width {
            self.ball_pos.x = screen_width * 0.5;
            self.ball_pos.y = screen_height * 0.5;
            randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
            self.player_one_score += 1;
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let racket_rect = graphics::Rect::new(-RACKET_WIDTH_HALF, -RACKET_HEIGHT_HALF, RACKET_WIDTH, RACKET_HEIGHT);
        let racket_rect_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(4.0), racket_rect, graphics::WHITE)?;

        let ball_rect = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_rect_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(4.0), ball_rect, graphics::WHITE)?;

        let mut draw_param = graphics::DrawParam::default();
        
        draw_param.dest = self.player_one_pos.into();
        graphics::draw(ctx, &racket_rect_mesh, draw_param);


        draw_param.dest = self.player_two_pos.into();
        graphics::draw(ctx, &racket_rect_mesh, draw_param);

        draw_param.dest = self.ball_pos.into();
        graphics::draw(ctx, &ball_rect_mesh, draw_param);

        let score_text = graphics::Text::new(format!("{}          {}", self.player_one_score, self.player_two_score));
        let (screen_width, screen_height) = graphics::drawable_size(ctx);
        let (screen_width_half, screen_height_half) = (screen_width * 0.5, screen_height * 0.5);

        let score_pos = na::Point2::new(screen_width_half, screen_height_half);
        draw_param.dest = score_pos.into();

        graphics::draw(ctx, &score_text, draw_param)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Pong_0", "Vic");
    let (ctx, event_loop) = &mut cb.build()?;
    graphics::set_window_title(ctx, "PONG");

    let mut state = MainState::new(ctx);

    event::run(ctx, event_loop, &mut state);
    Ok(())
}
