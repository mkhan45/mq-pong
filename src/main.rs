use macroquad::prelude::*;

const PADDLE_SPEED: f32 = 5.0;

struct MainState {
    top_paddle: Rect,
    bottom_paddle: Rect,
    ball: Rect,
    ball_vel: Vec2,
    top_score: usize,
    bottom_score: usize,
}

impl MainState {
    fn new() -> Self {
        MainState {
            top_paddle: Rect::new(screen_width() / 2.0, 10.0, 200.0, 10.0),
            bottom_paddle: Rect::new(screen_width() / 2.0, screen_height() - 30.0, 200.0, 10.0),
            ball: Rect::new(screen_width() / 2.0, screen_height() / 2.0, 10.0, 10.0),
            ball_vel: Vec2::new(2.0, 5.0),
            top_score: 0,
            bottom_score: 0,
        }
    }

    fn reset_ball(&mut self) {
        self.ball_vel.x *= -1.0;
        self.ball_vel.y *= -1.0;
        self.ball.move_to(Vec2::new(screen_width() / 2.0, screen_height() / 2.0));
    }

    fn update(&mut self) {
        if is_key_down(KeyCode::A) {
            self.top_paddle.x -= PADDLE_SPEED;
        }
        if is_key_down(KeyCode::D) {
            self.top_paddle.x += PADDLE_SPEED;
        }

        if is_key_down(KeyCode::Left) {
            self.bottom_paddle.x -= PADDLE_SPEED;
        }
        if is_key_down(KeyCode::Right) {
            self.bottom_paddle.x += PADDLE_SPEED;
        }

        self.ball.x += self.ball_vel.x;
        self.ball.y += self.ball_vel.y;

        if self.ball.overlaps(&self.bottom_paddle) 
        || self.ball.overlaps(&self.top_paddle) {
            self.ball_vel.y *= -1.0;
        }

        if self.ball.left() <= 0.0 
        || self.ball.right() >= screen_width() {
            self.ball_vel.x *= -1.0;
        }

        if self.ball.top() <= 0.0 {
            self.bottom_score += 1;
            self.reset_ball();
        }
        if self.ball.bottom() >= screen_height() {
            self.top_score += 1;
            self.reset_ball();
        }
    }

    fn draw_rect(&self, rect: &Rect) {
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            WHITE,
        )
    }

    fn draw(&self) {
        self.draw_rect(&self.top_paddle);
        self.draw_rect(&self.bottom_paddle);
        self.draw_rect(&self.ball);

        draw_text(&self.top_score.to_string(), 20.0, 20.0, 40.0, WHITE);
        draw_text(&self.bottom_score.to_string(), 20.0, screen_height() - 20.0, 40.0, WHITE);
    }
}

#[macroquad::main("Pong")]
async fn main() {
    let mut main_state = MainState::new();

    loop {
        clear_background(BLACK);
        main_state.update();
        main_state.draw();
        next_frame().await
    }
}
