use std::io::stdin;
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;

const COAT_SIZE: u32 = 64;
const HALF_PADDLE_SIZE: f64 = 0.2/2.0;

pub struct Game {
    pub ball_position: f64,
    pub speed: f64,
}

impl Game {
    pub fn new() -> Self {
        Self {
            ball_position: 0.0,
            speed: 0.01,
        }
    }

    pub fn update(&mut self, is_swing: &Mutex<bool>) -> bool {
        let mut is_swing = is_swing.lock().unwrap();
        let is_hit_left = if *is_swing && (-HALF_PADDLE_SIZE..HALF_PADDLE_SIZE).contains(&self.ball_position) {true} else {false}; 
        let is_hit_right = if *is_swing && (1.0-HALF_PADDLE_SIZE..1.0+HALF_PADDLE_SIZE).contains(&self.ball_position) {true} else {false}; 

        if is_hit_left || is_hit_right {self.speed *= -1.0};
        self.ball_position += self.speed;
        *is_swing = false;

        let is_out_left = self.ball_position < (0.0 - HALF_PADDLE_SIZE);
        let is_out_right = self.ball_position > (1.0 + HALF_PADDLE_SIZE);

        !(is_out_left || is_out_right)
    }
}

fn game_loop(game: &mut Game, is_swing: &Mutex<bool>) {
    loop {
        if !game.update(&is_swing) {
            break;
        };
        draw(game.ball_position);
        let dur = Duration::from_millis(17);
        sleep(dur);
    }

    println!("Game over!");
}

fn draw(ball: f64) {
    let ball= (COAT_SIZE as f64 * ball).round() as u32;
    let mut buf = String::new();
    buf += "|";

    for i in 0..COAT_SIZE {
        buf += if i == ball { "*" } else { " " };
    }
    buf += "|";

    println!("\x1B[1;1H{}", buf);
}

fn sub_main(is_swing: &Mutex<bool>) -> ! {
    let input = stdin();
    let mut buf = String::new();
    loop {
        input.read_line(&mut buf).expect("input error!");
        *is_swing.lock().unwrap() = true;
    }
}

fn main() {
    println!("\x1B[2J");
    let is_swing: Arc<Mutex<bool>> = Default::default();
    {
        let is_swing = is_swing.clone();
        spawn(move || sub_main(&is_swing));
    }
        let mut game = Game::new();
    game_loop(&mut game, &is_swing);
}
