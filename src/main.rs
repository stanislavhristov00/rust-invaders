mod entities;
mod input;
mod state;
mod init;

use std::path::{MAIN_SEPARATOR, Path};
use std::{env, path, fs};

use entities::{Enemy, Player};
use ggez::glam::Vec2;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Image, DrawParam, Rect, Canvas, Drawable};
use ggez::event::{self};
use ggez::conf::{Conf, WindowMode};
use state::State;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, Error};
use std::os::windows::prelude::*;
use winapi::um::winnt::FILE_ATTRIBUTE_HIDDEN;

const WINDOW_SIZE_HEIGHT: f32 = 1080.0;
const WINDOW_SIZE_WIDTH: f32 = 1920.0;

struct MainState{
    count: u64,
    state: Box<state::State>,
    state_copy: Box<state::State>,
    is_on_starting_screen: bool,
    is_game_over: bool,
    score: u64,
    high_score: u64,
    sprite_sheet: Image,
    score_file: String
}

impl MainState {
    pub fn new(_ctx: &mut Context, sprite_sheet: Image, score_file: &str) -> Self {
        let score_file_path = Path::new(score_file);

        let mut high_score = 0;
        if score_file_path.exists() {
            let high_score_string = fs::read_to_string(score_file_path);
            if let Err(err) = high_score_string {
                println!("Failed to read file {}", score_file);
            } else {
                match high_score_string.unwrap().parse::<u64>() {
                    Err(err) => { println!("Failed to parse high score? {}", err) },
                    Ok(num) => { high_score = num; }
                }
            }
        } else {
            // For windows we must set the hidden attribute ourselves
            if std::env::consts::OS == "windows" {
                let file = OpenOptions::new()
                           .write(true)
                           .create(true)
                           .attributes(FILE_ATTRIBUTE_HIDDEN)
                           .open(score_file);

                if let Err(err) = file {
                    println!("Failed to create high score file: {}", err.to_string());
                }
            } else {
                let file = OpenOptions::new()
                           .write(true)
                           .create(true)
                           .open(score_file);

                if let Err(err) = file {
                    println!("Failed to create high score file: {}", err.to_string());
                }
            }

        }

        let enemies = init::init_enemies(&sprite_sheet,
                                                    7,
                                                    Vec2 { x: 0.55, y: 0.55 },
                                                    WINDOW_SIZE_WIDTH,
                                                    WINDOW_SIZE_HEIGHT);

        let player = init::init_player(&sprite_sheet,
                                                    Vec2 { x: 1.0, y: 1.0 },
                                                    WINDOW_SIZE_WIDTH,
                                                    WINDOW_SIZE_HEIGHT);

        MainState {
            count: 0,
            state: Box::new( State::new(21, enemies.clone(), player.clone())),
            state_copy: Box::new( State::new(21, enemies, player)),
            is_on_starting_screen: true,
            is_game_over: false,
            score: 0,
            high_score: high_score,
            sprite_sheet: sprite_sheet,
            score_file: String::from(score_file)
        }
    }
}

impl MainState {
    pub fn draw_text(&mut self, canvas: &mut Canvas, ctx: &mut Context, is_game_over: bool, is_on_starting_screen: bool) {
        if is_on_starting_screen {
            let mut title = graphics::Text::new("SPACE INVADERS");
            title.set_font("MainFont");
            title.set_scale(graphics::PxScale::from(150.0));
            canvas.draw(&title, DrawParam::default().dest(Vec2::new(
                (WINDOW_SIZE_WIDTH - title.dimensions(ctx).unwrap().w) / 2.0,
                (WINDOW_SIZE_HEIGHT - title.dimensions(ctx).unwrap().h) / 3.0)));

            let mut desc_1 = graphics::Text::new("Move  with  ARROW  KEYS");
            desc_1.set_font("MainFont");
            desc_1.set_scale(graphics::PxScale::from(60.0));
            canvas.draw(&desc_1, DrawParam::default().dest(Vec2::new(
                (WINDOW_SIZE_WIDTH - desc_1.dimensions(ctx).unwrap().w) / 2.0,
                (WINDOW_SIZE_HEIGHT - desc_1.dimensions(ctx).unwrap().h) / 2.0)));

            let mut desc_2 = graphics::Text::new("Shoot  with  SPACEBAR");
            desc_2.set_font("MainFont");
            desc_2.set_scale(graphics::PxScale::from(60.0));
            canvas.draw(&desc_2, DrawParam::default().dest(Vec2::new(
                (WINDOW_SIZE_WIDTH - desc_2.dimensions(ctx).unwrap().w) / 2.0,
                (WINDOW_SIZE_HEIGHT - desc_2.dimensions(ctx).unwrap().h) / 1.8)));

            let mut desc_3 = graphics::Text::new("PRESS  SPACEBAR  TO  START");
            desc_3.set_font("MainFont");
            desc_3.set_scale(graphics::PxScale::from(70.0));
            canvas.draw(&desc_3, DrawParam::default().dest(Vec2::new(
                (WINDOW_SIZE_WIDTH - desc_3.dimensions(ctx).unwrap().w) / 2.0,
                (WINDOW_SIZE_HEIGHT - desc_3.dimensions(ctx).unwrap().h) / 1.5)));
        } else if is_game_over {
            let mut game_over = graphics::Text::new("GAME OVER");
            game_over.set_font("MainFont");
            game_over.set_scale(graphics::PxScale::from(150.0));
            canvas.draw(&game_over, DrawParam::default().dest(Vec2::new(
                (WINDOW_SIZE_WIDTH - game_over.dimensions(ctx).unwrap().w) / 2.0,
                (WINDOW_SIZE_HEIGHT - game_over.dimensions(ctx).unwrap().h) / 3.0)));

            let mut restart = graphics::Text::new("PRESS  SPACEBAR  TO  RESTART");
            restart.set_font("MainFont");
            restart.set_scale(graphics::PxScale::from(70.0));
            canvas.draw(&restart, DrawParam::default().dest(Vec2::new(
                (WINDOW_SIZE_WIDTH - restart.dimensions(ctx).unwrap().w) / 2.0,
                (WINDOW_SIZE_HEIGHT - restart.dimensions(ctx).unwrap().h) / 1.5)));
        } else {
            let mut lives = graphics::Text::new(format!("LIVES  {}", self.state.get_player_lives()));
            lives.set_font("MainFont");
            lives.set_scale(graphics::PxScale::from(40.0));

            canvas.draw(&lives, DrawParam::default().dest(Vec2::new(
                WINDOW_SIZE_WIDTH / 20.0,
                WINDOW_SIZE_HEIGHT / 50.0
            )));

            let mut score = graphics::Text::new(format!("SCORE  {}", self.score));
            score.set_font("MainFont");
            score.set_scale(graphics::PxScale::from(40.0));

            canvas.draw(&score, DrawParam::default().dest(Vec2::new(
                WINDOW_SIZE_WIDTH / 1.5,
                WINDOW_SIZE_HEIGHT / 50.0
            )));

            let mut high_score = graphics::Text::new(format!("HIGH  SCORE  {}", self.high_score));
            high_score.set_font("MainFont");
            high_score.set_scale(graphics::PxScale::from(40.0));

            canvas.draw(&high_score, DrawParam::default().dest(Vec2::new(
                WINDOW_SIZE_WIDTH - high_score.dimensions(ctx).unwrap().w * 1.5,
                WINDOW_SIZE_HEIGHT / 50.0
            )))
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.is_on_starting_screen {
            if input::is_space_pressed(ctx) {
                self.is_on_starting_screen = false;
            }
        } else if !self.is_game_over {
            self.count += 1;
            let image_dimensions = Vec2 { x: self.sprite_sheet.height() as f32, y: self.sprite_sheet.width() as f32};

            self.is_game_over = !self.state.is_player_alive();
            self.state.check_if_enemy_shot_player(image_dimensions.clone());



            if self.state.check_if_player_shot_enemy(image_dimensions.clone()) {
                self.score += 10;
            }

            if input::update_shoot(ctx) {
                self.state.player_shoot();
            }

            let direction = input::update_movement(ctx);

            // if direction != 0 {
            //     self.state.move_player(6.0 * direction as f32,
            //                            WINDOW_SIZE_WIDTH,
            //                            image_dimensions.clone(),
            //                            true /* scaled */);
            // }

            // self.state.move_enemies(WINDOW_SIZE_WIDTH, true, image_dimensions.clone());

            self.state.update(6.0 * direction as f32, image_dimensions, true, self.count);

            self.state.enemies_shoot();

            // if self.state.all_enemies_dead() {
            //     todo!();
            // }

        } else {

        }
        // Променяме състоянието на играта
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let black = graphics::Color::from_rgb(0, 0, 0);
        let mut canvas = graphics::Canvas::from_frame(ctx, black);

        if self.is_on_starting_screen {

        } else if !self.is_game_over {
            self.state.draw_enemies(&mut canvas, self.count, &self.sprite_sheet);
            self.state.draw_player(&mut canvas, self.count, &self.sprite_sheet);
        } else {

        }

        self.draw_text(&mut canvas, ctx, self.is_game_over, self.is_on_starting_screen);

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() {
    // Конфигурация:
    let conf = Conf::new().
        window_mode(WindowMode {
            width: WINDOW_SIZE_WIDTH,
            height: WINDOW_SIZE_HEIGHT,
            min_width: WINDOW_SIZE_WIDTH,
            min_height: WINDOW_SIZE_HEIGHT,
            ..Default::default()
        });

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("src");
        path.push("resources");
        path
    } else {
        path::PathBuf::from(format!(".{}src{}resources", MAIN_SEPARATOR, MAIN_SEPARATOR))
    };

    let cb = ggez::ContextBuilder::new("Space Invaders", "Stanislav Hristov").
                             default_conf(conf).
                             add_resource_path(resource_dir.clone());

    let (mut ctx, event_loop) = cb.build().unwrap();

    ctx.gfx.set_window_title("(Almost) Space Invaders!");

    let image = Image::from_path(&ctx, "/space_invaders.png");

    if let Err(err) = image {
        eprintln!("{err}");
        return;
    }

    init::init_font(&mut ctx, "/font.TTF");

    let mut high_score_file_path = String::from(resource_dir.into_os_string().to_str().unwrap());
    high_score_file_path += "/.high_score.txt";

    // Пускане на главния loop
    let state = MainState::new(&mut ctx, image.unwrap(), high_score_file_path.as_str());
    event::run(ctx, event_loop, state);
}