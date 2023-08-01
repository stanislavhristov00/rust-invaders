use crate::WINDOW_SIZE_WIDTH;
use crate::WINDOW_SIZE_HEIGHT;
use crate::entities;
use crate::input;
use core::sync::atomic::{AtomicU8, AtomicI8, Ordering};
use rand::Rng;

pub const NUM_ROWS: u8 = 3;

static NUM_ENEMIES_ON_ROW: AtomicU8 = AtomicU8::new(0);
static ENEMY_MOVEMENT_DIRECTION: AtomicI8 = AtomicI8::new(1);
static ENEMY_SPEED: AtomicU8 = AtomicU8::new(2);

use entities::Enemy;
use entities::Player;
use ggez::Context;
use ggez::glam::Vec2;
use ggez::graphics::Canvas;
use ggez::graphics::Image;
pub struct State {
   enemies: Vec<Enemy>,
   player: Box<Player>,
   num_enemies: u8
}

impl State {
   pub fn new(num_enemies: u8, enemies: Vec<Enemy>, player: Box<Player>) -> Self {
      NUM_ENEMIES_ON_ROW.store(num_enemies / NUM_ROWS, Ordering::Relaxed);

      State {
         enemies: enemies,
         num_enemies: num_enemies,
         player: player
      }
   }

   pub fn check_player_movement(&self, ctx: &Context) -> i8 {
      input::update_movement(ctx)
   }

   pub fn check_player_shoot(&self, ctx: &Context) -> bool {
      input::update_shoot(ctx)
   }

   pub fn draw_enemies(&mut self, canvas: &mut Canvas, count: u64, image: &Image) {
      for enemy in self.enemies.as_mut_slice() {
         enemy.draw(canvas, count, image);

         if !enemy.is_alive() && !enemy.get_death_frame_drawn() {
            // This also sets the death_frame_drawn to true
            enemy.draw_die(canvas, count, image);
         }
      }
   }

   pub fn all_enemies_dead(&mut self) -> bool {
      for enemy in self.enemies.as_mut_slice() {
         if enemy.is_alive() {
            return false;
         }
      }

      true
   }

   pub fn revive_player(&mut self, position: Vec2) {
      self.player.revive(position);
   }

   pub fn draw_player(&mut self, canvas: &mut Canvas, count: u64, image: &Image) {
      if self.player.is_alive() {
         self.player.draw(canvas, count, image);
      } else {
         self.player.draw_die(canvas, count, image);
      }
   }

   pub fn check_if_player_shot_enemy(&mut self, image_dimensions: Vec2) -> bool {
      for enemy in self.enemies.as_mut_slice() {
         if enemy.is_alive() {
            if self.player.bullet_collision_with_enemy(&enemy, image_dimensions) {
               self.player.set_bullet_in_air(false);
               enemy.die();
               return true;
            }
         }
      }

      return false;
   }

   pub fn check_if_enemy_shot_player(&mut self, image_dimensions: Vec2) -> bool {
      for enemy in self.enemies.as_mut_slice() {
         if enemy.bullet_collision_with_player(&self.player, image_dimensions) {
            self.player.lose_life();
            enemy.set_bullet_in_air(false);
            return true;
         }
      }

      return false;
   }

   pub fn move_enemies(&mut self, screen_width: f32, scaled: bool, image_dimensions: Vec2) {
      let first_enemy_on_row_coords = self.enemies[0].get_coords();
      let last_enemy_on_row = &self.enemies[(NUM_ENEMIES_ON_ROW.load(Ordering::Relaxed) - 1) as usize];
      let last_enemy_on_row_coords = self.enemies[(NUM_ENEMIES_ON_ROW.load(Ordering::Relaxed) - 1) as usize].get_coords();

      if first_enemy_on_row_coords.x < 0.0 || last_enemy_on_row_coords.x + image_dimensions.y * last_enemy_on_row.get_scale().y * last_enemy_on_row.get_frame_dimensions().unwrap().y > screen_width {
         let current_enemy_movement = ENEMY_MOVEMENT_DIRECTION.load(Ordering::Relaxed);
         ENEMY_MOVEMENT_DIRECTION.store(current_enemy_movement * -1, Ordering::Relaxed);


         // Move the enemies one row down.
         for enemy in self.enemies.as_mut_slice() {
            let dim = enemy.get_frame_dimensions().unwrap();
            enemy.translate(Vec2::new(0.0, dim.x * image_dimensions.x), scaled);
         }
      }

      for enemy in self.enemies.as_mut_slice() {
         let x: f32 = ENEMY_MOVEMENT_DIRECTION.load(Ordering::Relaxed) as f32 * ENEMY_SPEED.load(Ordering::Relaxed) as f32;
         enemy.translate(Vec2::new(x, 0.0), scaled);

         // if enemy.is_alive() && enemy.get_coords().y >= self.player.get_coords().y {
         //    self.player.die();
         // }
      }
   }

   pub fn enemies_shoot(&mut self) {
      for enemy in self.enemies.as_mut_slice() {
         if enemy.is_alive() && enemy.is_bullet_in_air() {
            return;
         }
      }

      let mut enemies_alive = Vec::new();

      for enemy in self.enemies.as_mut_slice() {
         if enemy.is_alive() {
            enemies_alive.push(enemy);
         }
      }

      if enemies_alive.len() == 0 {
         return;
      }

      if enemies_alive.len() <= 5 {
         for alive in enemies_alive {
            alive.shoot();
         }

         return;
      }

      let mut enemies_to_shoot: u16 = 0;

      // These values can be played around with :)
      if enemies_alive.len() > 20 {
         enemies_to_shoot = 5;
      } else {
         enemies_to_shoot = 3;
      }

      for i in 0..enemies_to_shoot {
         let mut index = rand::thread_rng().gen_range(0..enemies_to_shoot);

         let mut enemy = &mut self.enemies.as_mut_slice()[index as usize];

         if enemy.is_bullet_in_air() {
            index = rand::thread_rng().gen_range(0..enemies_to_shoot);

            enemy = &mut self.enemies.as_mut_slice()[index as usize];
            if enemy.is_bullet_in_air() {
               continue;
            } else {
               enemy.shoot();
            }
         } else {
            enemy.shoot();
         }
      }
   }

   pub fn move_player(&mut self, x: f32, screen_width: f32, image_dimensions: Vec2, scaled: bool) {
      let player_coords = self.player.get_coords();
      let player_dim = self.player.get_frame_dimensions().unwrap();

      if player_coords.x + x < 0.0 ||
         player_coords.x + image_dimensions.y * player_dim.y * self.player.get_scale().y  + x >= screen_width {
         return;
      }

      self.player.translate(Vec2::new(x, 0.0), scaled);
   }

   pub fn set_enemies(&mut self, enemies: Vec<Enemy>) {
      self.enemies = enemies;
   }

   pub fn player_shoot(&mut self) {
      self.player.shoot();
   }

   pub fn is_player_alive(&self) -> bool {
      self.player.is_alive()
   }

   pub fn update(&mut self, x: f32, image_dimensions: Vec2, scaled: bool, count: u64) {
      self.player.update(9.0, image_dimensions, count, scaled);

      for enemy in self.enemies.as_mut_slice() {
         enemy.update(6.0, WINDOW_SIZE_HEIGHT, image_dimensions, count, scaled);
      }

      self.move_enemies(WINDOW_SIZE_WIDTH, scaled, image_dimensions);
      self.move_player(x, WINDOW_SIZE_WIDTH, image_dimensions, scaled)
   }

   pub fn get_player_lives(&self) -> u8 {
      self.player.get_lives()
   }

   pub fn get_enemies(&self) -> Vec<Enemy> {
      self.enemies.clone()
   }
}

pub fn set_enemy_direction_right() {
   let current_value = ENEMY_MOVEMENT_DIRECTION.load(Ordering::Relaxed);

   ENEMY_MOVEMENT_DIRECTION.store(current_value * -1, Ordering::Relaxed);
}

pub fn reset_enemy_movement_speed() {
   ENEMY_SPEED.store(2, Ordering::Relaxed);
}

pub fn increase_enemy_movement_speed(speed: u8) {
   let current_value = ENEMY_SPEED.load(Ordering::Relaxed);

   ENEMY_SPEED.store(current_value + speed, Ordering::Relaxed);
}