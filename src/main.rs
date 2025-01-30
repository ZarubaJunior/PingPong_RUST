use ggez::event;
use ggez::input::keyboard::{self, KeyCode};
use ggez::nalgebra as na;
use ggez::{self, graphics};
use ggez::{Context, GameResult};
use rand::prelude::*;

const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_HEIGHT_HALF: f32 = 100.0 * 0.5;
const RACKET_WIDTH_HALF: f32 = 20.0 * 0.5;
const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const MIDDLE_LINE_WiDTH: f32 = 10.0;
const PLAYER_SPEED: f32 = 600.0;
const BALL_SPEED_CONST: f32 = 300.0;

fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

fn ball_start_pos() -> f32 {
    let mut rng = rand::thread_rng();
    let random_bool: bool = rng.gen();

    println!("{}", random_bool);

    if random_bool == true {
        -BALL_SPEED_CONST
    } else {
        BALL_SPEED_CONST
    }
}

struct MainState {
    player_1_pos: na::Point2<f32>,
    player_2_pos: na::Point2<f32>,
    ball_pos: na::Point2<f32>,
    middle_line_pos: na::Point2<f32>,
    ball_speed_y: f32,
    ball_speed_x: f32,
    player_1_score: u16,
    player_2_score: u16,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);
        MainState {
            player_1_pos: na::Point2::new(RACKET_WIDTH_HALF, screen_h_half),
            player_2_pos: na::Point2::new(screen_w - RACKET_WIDTH_HALF, screen_h_half),
            ball_pos: na::Point2::new(screen_w_half, screen_h_half),
            middle_line_pos: na::Point2::new(screen_w_half, screen_h_half),
            ball_speed_y: ball_start_pos(),
            ball_speed_x: ball_start_pos(),
            player_1_score: 0,
            player_2_score: 0,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        let screen_h = graphics::drawable_size(ctx).1;
        let screen_h_half = screen_h * 0.5;
        let screen_w = graphics::drawable_size(ctx).0;
        let screen_w_half = screen_w * 0.5;

        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.player_1_pos.y -= PLAYER_SPEED * dt;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.player_1_pos.y += PLAYER_SPEED * dt;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.player_1_pos.x -= PLAYER_SPEED * dt;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.player_1_pos.x += PLAYER_SPEED * dt;
        }
        clamp(
            &mut self.player_1_pos.y,
            RACKET_HEIGHT_HALF,
            screen_h - RACKET_HEIGHT_HALF,
        );
        clamp(
            &mut self.player_1_pos.x,
            RACKET_WIDTH_HALF,
            screen_w_half - RACKET_WIDTH_HALF,
        );

        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.player_2_pos.y -= PLAYER_SPEED * dt;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.player_2_pos.y += PLAYER_SPEED * dt;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            self.player_2_pos.x -= PLAYER_SPEED * dt;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            self.player_2_pos.x += PLAYER_SPEED * dt;
        }
        clamp(
            &mut self.player_2_pos.y,
            RACKET_HEIGHT_HALF,
            screen_h - RACKET_HEIGHT_HALF,
        );
        clamp(
            &mut self.player_2_pos.x,
            screen_w_half + RACKET_WIDTH_HALF,
            screen_w - RACKET_WIDTH_HALF,
        );

        if self.ball_pos.x <= self.player_1_pos.x + RACKET_WIDTH_HALF
            && self.ball_pos.x >= self.player_1_pos.x - RACKET_WIDTH_HALF
            && self.ball_pos.y > self.player_1_pos.y - RACKET_HEIGHT_HALF
            && self.ball_pos.y < self.player_1_pos.y + RACKET_HEIGHT_HALF
        {
            self.ball_speed_x = -self.ball_speed_x;
        }

        if self.ball_pos.x <= self.player_2_pos.x + RACKET_WIDTH_HALF
            && self.ball_pos.x >= self.player_2_pos.x - RACKET_WIDTH_HALF
            && self.ball_pos.y > self.player_2_pos.y - RACKET_HEIGHT_HALF
            && self.ball_pos.y < self.player_2_pos.y + RACKET_HEIGHT_HALF
        {
            self.ball_speed_x = -self.ball_speed_x;
        }

        self.ball_pos.x += self.ball_speed_x * dt;

        if self.ball_pos.x <= 0.0 {
            self.player_2_score += 1;
            self.ball_pos.x = screen_w_half;
            self.ball_pos.y = screen_h_half;
            self.ball_speed_y = ball_start_pos();
            self.ball_speed_x = ball_start_pos();
        }
        if self.ball_pos.x >= screen_w {
            self.player_1_score += 1;
            self.ball_pos.x = screen_w_half;
            self.ball_pos.y = screen_h_half;
            self.ball_speed_y = ball_start_pos();
            self.ball_speed_x = ball_start_pos();
        }

        if self.ball_pos.y <= 0.0 + BALL_SIZE_HALF {
            self.ball_speed_y = -self.ball_speed_y;
        } else if self.ball_pos.y >= screen_h - BALL_SIZE_HALF {
            self.ball_speed_y = -self.ball_speed_y;
        }
        self.ball_pos.y += self.ball_speed_y * dt;

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);

        graphics::clear(ctx, graphics::BLACK); // background color

        let racket_rect = graphics::Rect::new(
            -RACKET_WIDTH_HALF,
            -RACKET_HEIGHT_HALF,
            RACKET_WIDTH,
            RACKET_HEIGHT,
        );
        let rect_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(4.0),
            racket_rect,
            graphics::WHITE,
        )?;

        let ball_rect = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ball_rect,
            graphics::WHITE,
        )?;

        let middle_line_rect = graphics::Rect::new(
            -MIDDLE_LINE_WiDTH + MIDDLE_LINE_WiDTH * 0.5,
            -screen_h * 0.5,
            MIDDLE_LINE_WiDTH,
            screen_h,
        );
        let middle_line_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            middle_line_rect,
            graphics::WHITE,
        )?;

        let mut draw_param = graphics::DrawParam::default();

        draw_param.dest = self.player_1_pos.into();
        graphics::draw(ctx, &rect_mesh, draw_param)?;

        draw_param.dest = self.player_2_pos.into();
        graphics::draw(ctx, &rect_mesh, draw_param)?;

        draw_param.dest = self.ball_pos.into();
        graphics::draw(ctx, &ball_mesh, draw_param)?;

        draw_param.dest = self.middle_line_pos.into();
        graphics::draw(ctx, &middle_line_mesh, draw_param)?;

        let score_text = graphics::Text::new(format!(
            "{}         {}",
            self.player_1_score, self.player_2_score
        ));

        let mut score_pos = na::Point2::new(screen_w_half, 40.0);
        let (score_text_w, score_text_h) = score_text.dimensions(ctx);
        score_pos -= na::Vector2::new(score_text_w as f32 * 0.5, score_text_h as f32 * 0.5);
        draw_param.dest = score_pos.into();

        graphics::draw(ctx, &score_text, draw_param)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Ping_Pong_0", "Zaruba_Junior"); // project name
    let (ctx, event_loop) = &mut cb.build()?;
    graphics::set_window_title(ctx, "PONG");

    let mut state = MainState::new(ctx);
    event::run(ctx, event_loop, &mut state);
    Ok(())
}
