use ggez::{Context, graphics::{self, Image, DrawParam, Rect}, glam::Vec2};

use crate::entities::{ Enemy, Sprite, Player };
use crate::state::NUM_ROWS;

pub fn init_font(ctx: &mut Context, path: &str) {
   let font_data = graphics::FontData::from_path(ctx, path).unwrap();
   ctx.gfx.add_font("MainFont", font_data);
}

//Player: Rect::new(0.25, 0.9, 0.16, 0.1)
//Player death frame: Rect::new(0.40, 0.9, 0.19, 0.12)
pub fn init_player(image: &Image, scale: Vec2, screen_width: f32, screen_height: f32) -> Box<Player> {
   let image_dimensions = Vec2::new(image.height() as f32, image.width() as f32);
   let mut player_instance_array = Vec::new();
   player_instance_array.push(DrawParam::default().src(Rect::new(0.25, 0.9, 0.16, 0.1)));

   let mut player_death_instance_array = Vec::new();
   player_death_instance_array.push(DrawParam::default().src(Rect::new(0.40, 0.9, 0.19, 0.12)));

   let mut bullet_instance_array = Vec::new();
   bullet_instance_array.push(DrawParam::default().src(Rect::new(0.83, 0.55, 0.15, 0.12)));

   let sprite_alive = Sprite::new(player_instance_array);

   let frame_dimension = sprite_alive.get_frame_dimensions(0).unwrap();

   Box::new(
      Player::new(
         sprite_alive,
         Sprite::new(player_death_instance_array),
         Sprite::new(bullet_instance_array),
         Rect {
            x: screen_width / 2.2 - (frame_dimension.y * scale.x * image_dimensions.y / 2.0),
            y: screen_height - (frame_dimension.x * scale.y * image_dimensions.x + 10.0),
            w: scale.x,
            h: scale.y }
      )
   )
}

pub fn init_enemies(image: &Image, num_enemies_on_row: u32, scale: Vec2, screen_width: f32, screen_height: f32) -> Vec<Enemy> {
   //Enemy 1:  Rect::new(0.0, 0.0, 0.25, 0.12), Rect::new(0.25, 0.0, 0.25, 0.12)
   //Enemy 2:  Rect::new(0.55, 0.0, 0.20, 0.12), Rect::new(0.76, 0.0, 0.20, 0.12)
   //Enemy 3:  Rect::new(0.0, 0.175, 0.25, 0.12), Rect::new(0.25, 0.175, 0.25, 0.12)
   //Enemy death frame: Rect::new(0.62, 0.9, 0.20, 0.12)
   //Bullet: Rect::new(0.83, 0.55, 0.15, 0.12)

   let mut enemies: Vec<Enemy> = Vec::new();
   let image_dimensions = Vec2::new(image.height() as f32, image.width() as f32);

   let mut enemy1_instance_array = Vec::new();
   enemy1_instance_array.push(DrawParam::default().src(Rect::new(0.0, 0.0, 0.25, 0.12)));
   enemy1_instance_array.push(DrawParam::default().src(Rect::new(0.25, 0.0, 0.25, 0.12)));

   let enemy1_sprite_alive = Sprite::new(enemy1_instance_array);

   // let mut enemy2_instance_array = Vec::new();
   // enemy2_instance_array.push(DrawParam::default().src(Rect::new(0.55, 0.0, 0.20, 0.12)));
   // enemy2_instance_array.push(DrawParam::default().src(Rect::new(0.76, 0.0, 0.20, 0.12)));

   // let enemy2_sprite_alive = Sprite::new(enemy2_instance_array);

   let mut enemy3_instance_array = Vec::new();
   enemy3_instance_array.push(DrawParam::default().src(Rect::new(0.0, 0.175, 0.25, 0.12)));
   enemy3_instance_array.push(DrawParam::default().src(Rect::new(0.25, 0.175, 0.25, 0.12)));

   let enemy3_sprite_alive = Sprite::new(enemy3_instance_array);

   let mut enemy_death_instance_array = Vec::new();
   enemy_death_instance_array.push(DrawParam::default().src(Rect::new(0.62, 0.9, 0.20, 0.12)));

   let enemy_death_sprite = Sprite::new(enemy_death_instance_array);

   let mut bullet_instance_array= Vec::new();
   bullet_instance_array.push(DrawParam::default().src(Rect::new(0.83, 0.55, 0.15, 0.12)));

   let bullet_sprite = Sprite::new(bullet_instance_array);
   let mut count = 0;

   let enemy1_dim = enemy1_sprite_alive.clone().get_frame_dimensions(0).unwrap();
   // let enemy2_dim = enemy2_sprite_alive.clone().get_frame_dimensions(0).unwrap();
   let enemy3_dim = enemy3_sprite_alive.clone().get_frame_dimensions(0).unwrap();

   // Figure out a way here to make the enemies init more left, the bigger they are :) For now it's like this
   for row in 0..NUM_ROWS {
      for enemy_count in 0..num_enemies_on_row {
         if count == 0 {
            enemies.push(Enemy::new(enemy1_sprite_alive.clone(),
                                    enemy_death_sprite.clone(),
                                    bullet_sprite.clone(),
                                    Rect {x: (screen_width / 3.0) + enemy_count as f32 * enemy1_dim.y * image_dimensions.y * scale.x,
                                                    y: screen_height / 12.0 + row as f32 * enemy1_dim.x * image_dimensions.x + ((row as f32 * (enemy1_dim.x * image_dimensions.x * scale.y)) / 2.0),
                                                    w: scale.x,
                                                    h: scale.y }));
         } else {
            enemies.push(Enemy::new(enemy3_sprite_alive.clone(),
                                    enemy_death_sprite.clone(),
                                    bullet_sprite.clone(),
                                    Rect {x: (screen_width / 3.0) + enemy_count as f32 * enemy3_dim.y * image_dimensions.y * scale.x,
                                                    y: screen_height / 12.0 + row as f32 * enemy3_dim.x * image_dimensions.x  + ((row as f32 * (enemy3_dim.x * image_dimensions.x * scale.y)) / 2.0),
                                                    w: scale.x,
                                                    h: scale.y }));
         }
      }

      count += 1;
      if count > 1 {
         count = 0;
      }
   }

   enemies
}