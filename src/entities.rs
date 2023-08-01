use ggez::{graphics::{Image, Canvas, Rect, DrawParam}, glam::Vec2, GameError};

const PLAYER_NUM_LIVES: u8 = 3;


// TODO!: Extract all common methods in a trait..
// TODO!: Make an entity for the walls..
// TODO!: Make an entity for the big ship that is worth a 100 points..

#[derive(Clone)]
/*
 * Structure that will handle animation of entities.
 */
pub struct Sprite {
   // Use Vec<> instead of InstanceArray here, since Vec implements the Clone
   // trait and InstanceArray doesn't.

   animationFrames: Vec<DrawParam>,
}

impl Sprite {
   pub fn new(animationFrames: Vec<DrawParam>) -> Self {
      Sprite {
         animationFrames: animationFrames
      }
   }

   pub fn draw(&self, canvas: &mut Canvas, counter: u64, dest_rect: Rect, image: &Image) {
      if self.animationFrames.len() == 0 {
         canvas.draw(image, self.animationFrames[0].dest_rect(dest_rect));
      } else {
         let frameIndex: usize = (counter as usize / 20) % self.animationFrames.len();
         canvas.draw(image, self.animationFrames[frameIndex].dest_rect(dest_rect));
      }
   }

   pub fn get_frame_dimensions(&self, index: usize) -> Result<Vec2, GameError> {
      if index >= self.animationFrames.len() {
         return Err(GameError::CustomError(String::from("Invalid index passed to get_frame_dimensions")));
      }

      Ok(Vec2 { x: self.animationFrames[index].src.h, y: self.animationFrames[index].src.w })
   }
}

#[derive(Clone)]
pub struct Bullet {
   sprite:    Sprite,
	dest_rect:  Rect,
	in_air:     bool
}

impl Bullet {
   pub fn new(sprite: Sprite, dest_rect: Rect) -> Self{
      Bullet {
         sprite: sprite,
         dest_rect: dest_rect,
         in_air: false
      }
   }

   /*
    * Translate the coordinates of the bullet. Scaled is whether you want
    * to take into account the scale of the image (the dest_rect width and height)
    * when calculating the offsets. For e.g if you have a dest_rect.w = 1.5,
    * and you want to move the sprite on the x axis with 1 and you set scaled to true,
    * it will actually move 1.5 units to the right. If scaled is set to false,
    * it will always translate exactly the amount specified.
    */
   pub fn translate(&mut self, offset: Vec2, scaled: bool) {
      if scaled {
         let x = offset.x;
         let y = offset.y;

         // if the scale is 1 to 1, translation will be 1 to 1
         // take into account the scale
         self.dest_rect.translate(Vec2::new(x * self.dest_rect.w, y * self.dest_rect.h));
      } else {
         self.dest_rect.translate(offset);
      }

   }

   pub fn get_coords(&self) -> Vec2 {
      Vec2 { x: self.dest_rect.x, y: self.dest_rect.y }
   }

   pub fn set_bullet_coords(&mut self, offset: Vec2) {
      self.dest_rect.x = offset.x;
      self.dest_rect.y = offset.y;
   }

   pub fn set_bullet_dest(&mut self, dest_rect: Rect) {
      self.dest_rect = dest_rect;
   }

   pub fn set_in_air(&mut self, in_air: bool) {
      self.in_air = in_air;
   }

   pub fn in_air(&self) -> bool {
      return self.in_air;
   }

   pub fn get_frame_dimensions(&self) -> Result<Vec2, GameError> {
      self.sprite.get_frame_dimensions(0)
   }

   pub fn get_scale(&self) -> Vec2 {
      Vec2 { x: self.dest_rect.h, y: self.dest_rect.w }
   }

   pub fn draw(&self, canvas: &mut Canvas, counter: u64,image: &Image) {
      self.sprite.draw(canvas, counter, self.dest_rect, image);
   }
}

#[derive(Clone)]
pub struct Enemy {
   sprite_alive: Sprite,
	sprite_death: Sprite,
	bullet:      Bullet,
	dest_rect:    Rect,
	is_alive:     bool,
   death_animation_drawn: bool
}

impl Enemy {
   pub fn new(sprite_alive: Sprite, sprite_death: Sprite, bulletSprite: Sprite, dest_rect: Rect) -> Self {
      Enemy { sprite_alive: sprite_alive,
              sprite_death: sprite_death,
              bullet: Bullet { sprite: bulletSprite, dest_rect: dest_rect.clone(), in_air: false},
              dest_rect: dest_rect,
              is_alive: true,
              death_animation_drawn: false }
   }

   pub fn shoot(&mut self) {
      self.bullet.set_in_air(true);
   }

   pub fn update(&mut self, speed_y: f32, screen_height: f32, image_dimensions: Vec2, count: u64, scaled: bool) {
      if self.bullet.in_air() {
         self.bullet.translate(Vec2 { x: 0.0, y: speed_y }, scaled);

         if self.bullet.get_coords().y > screen_height {
            self.bullet.set_in_air(false);
         }
      } else {
         if self.is_alive {
            // Get on which frame we're currently
            let frame_index: usize = (count as usize / 20) % self.sprite_alive.animationFrames.len();
            let frame_dim = self.sprite_alive.get_frame_dimensions(frame_index).unwrap();

            // Calculate the offsets
            // let height_offset = image_dimensions.x as f32 * frame_dim.x;
            // let width_offset = image_dimensions.y as f32 * frame_dim.y;
            self.bullet.set_bullet_coords(Vec2::new(
                                          self.dest_rect.x, //+ width_offset,
                                          self.dest_rect.y)); //+ height_offset));
         }
      }
   }

   pub fn draw(&mut self, canvas: &mut Canvas, count: u64, image: &Image) {
      let screen_coords = canvas.screen_coordinates().unwrap();

      if self.bullet.in_air() {
        self.bullet.draw(canvas, count, image);
      }

      if self.is_alive {
         self.sprite_alive.draw(canvas, count, self.dest_rect, image);
      }
   }

   pub fn draw_die(&mut self, canvas: &mut Canvas, count: u64, image: &Image) {
      if !self.is_alive && !self.death_animation_drawn{
         self.sprite_death.draw(canvas, count, self.dest_rect, image);
         self.death_animation_drawn = true;
      }
   }

   pub fn get_coords(&self) -> Vec2 {
      Vec2 { x: self.dest_rect.x, y: self.dest_rect.y }
   }

   /*
    * Translate the coordinates of the enemy. Scaled is whether you want
    * to take into account the scale of the image (the dest_rect width and height)
    * when calculating the offsets. For e.g if you have a dest_rect.w = 1.5,
    * and you want to move the sprite on the x axis with 1 and you set scaled to true,
    * it will actually move 1.5 units to the right. If scaled is set to false,
    * it will always translate exactly the amount specified.
    */
   pub fn translate(&mut self, offset: Vec2, scaled: bool) {
      if scaled {
         let x = offset.x;
         let y = offset.y;

         self.dest_rect.translate(Vec2::new(x * self.dest_rect.w, y * self.dest_rect.h));
      } else {
         self.dest_rect.translate(offset);
      }
   }

   pub fn set_dest_rect(&mut self, rect: Rect) {
      self.dest_rect = rect;
   }

   pub fn die(&mut self) {
      self.is_alive = false;
   }

   pub fn get_frame_dimensions(&self) -> Result<Vec2, GameError> {
      self.sprite_alive.get_frame_dimensions(0)
   }

   pub fn get_death_frame_drawn(&self) -> bool {
      self.death_animation_drawn
   }

   pub fn is_alive(&self) -> bool {
      self.is_alive
   }

   pub fn get_scale(&self) -> Vec2 {
      Vec2 { x: self.dest_rect.h, y: self.dest_rect.w }
   }

   pub fn set_bullet_in_air(&mut self, in_air: bool) {
      self.bullet.set_in_air(in_air);
   }

   pub fn is_bullet_in_air(&self) -> bool {
      self.bullet.in_air()
   }

   pub fn bullet_collision_with_player(&self, player: &Player, image_dimensions: Vec2) -> bool {
      if self.bullet.in_air() {
         let bullet_coords = self.bullet.get_coords();
         let bullet_dim = self.bullet.get_frame_dimensions().unwrap();
         let bullet_scale = self.bullet.get_scale();

         let player_coords = player.get_coords();
         let player_dim = player.get_frame_dimensions().unwrap();
         let player_scaling = player.get_scale();

         // if bullet_coords.y >= player_coords.y - 10.0 {
         //    if bullet_coords.x >= player_coords.x - 10.0 &&
         //       bullet_coords.x <= player_coords.x + player_dimensions.y * image_dimensions.y - 10.0 {
         //       return true;
         //    }
         // }

         // if bullet_coords.y <= player_coords.y + player_dim.x * image_dimensions.x * player_scaling.x &&
         //    bullet_coords.y * bullet_dim.x * image_dimensions.x * bullet_scale.x >= player_coords.y + player_dim.x * image_dimensions.x * player_scaling.x {
         //    if (bullet_coords.x + 10.0 >= player_coords.x &&
         //       bullet_coords.x + 10.0 <= player_coords.x + player_dim.y * image_dimensions.y * player_scaling.y - 30.0)
         //       ||
         //       (bullet_coords.x + bullet_dim.y * image_dimensions.y * bullet_scale.y >= player_coords.x &&
         //        bullet_coords.x + bullet_dim.y * image_dimensions.y * bullet_scale.y <= player_coords.x + player_dim.y * image_dimensions.y * player_scaling.y - 30.0)
         //    {
         //       return true;
         //    }
         // }

         if bullet_coords.y >= player_coords.y && bullet_coords.y + bullet_dim.x * image_dimensions.x * bullet_scale.x >= player_coords.y {
            if (bullet_coords.x + 10.0 >= player_coords.x &&
               bullet_coords.x + 10.0 <= player_coords.x + player_dim.y * image_dimensions.y * player_scaling.y - 30.0)
               ||
               (bullet_coords.x + bullet_dim.y * image_dimensions.y * bullet_scale.y >= player_coords.x &&
                bullet_coords.x + bullet_dim.y * image_dimensions.y * bullet_scale.y <= player_coords.x + player_dim.y * image_dimensions.y * player_scaling.y - 30.0)
            {
               return true;
            }
         }
      }

      return false;
   }
}

#[derive(Clone)]
pub struct Player {
   sprite_alive: Sprite,
	sprite_death: Sprite,
	bullet:      Bullet,
	dest_rect:    Rect,
	is_alive:     bool,
	lives:        u8
}

impl Player {
   pub fn new(sprite_alive: Sprite, sprite_death: Sprite, bulletSprite: Sprite, dest_rect: Rect) -> Self {
      let mut dest_rect_bullet = dest_rect.clone();
      dest_rect_bullet.w /= 2.0;
      dest_rect_bullet.h /= 2.0;


      Player { sprite_alive: sprite_alive,
               sprite_death: sprite_death,
               bullet: Bullet { sprite: bulletSprite, dest_rect: dest_rect_bullet, in_air: false},
               dest_rect: dest_rect,
               is_alive: true,
               lives: PLAYER_NUM_LIVES}
   }

   pub fn shoot(&mut self) {
      self.bullet.set_in_air(true);
   }

   /*
    * TODO!. Refer to the translate method for the explanation of scaled.
    */
   pub fn update(&mut self, speed_y: f32, imageDimensions: Vec2, count: u64, scaled: bool) {
      if self.bullet.in_air() {
         self.bullet.translate(Vec2 { x: 0.0, y: -speed_y }, scaled);

         if self.bullet.get_coords().y < 0.0 {
            self.bullet.set_in_air(false);
         }
      } else {
         if self.is_alive {
            self.bullet.set_bullet_coords(Vec2::new(
                                          self.dest_rect.x,
                                          self.dest_rect.y));
         }
      }
   }

   pub fn draw(&mut self, canvas: &mut Canvas, count: u64, image: &Image) {
      let screen_coords = canvas.screen_coordinates().unwrap();

      if self.bullet.in_air() {
        self.bullet.draw(canvas, count, image);
      }

      if self.is_alive {
         self.sprite_alive.draw(canvas, count, self.dest_rect, image);
      }
   }

   pub fn is_alive(&self) -> bool {
      self.is_alive
   }

   pub fn draw_die(&mut self, canvas: &mut Canvas, count: u64, image: &Image) {
      if !self.is_alive {
         self.sprite_death.draw(canvas, count, self.dest_rect, image);
      }
   }

   pub fn get_coords(&self) -> Vec2 {
      Vec2 { x: self.dest_rect.x, y: self.dest_rect.y }
   }

   pub fn translate(&mut self, offset: Vec2, scaled: bool) {
      if scaled {
         let x = offset.x;
         let y = offset.y;

         self.dest_rect.translate(Vec2::new(x * self.dest_rect.w, y * self.dest_rect.y));
      }
      self.dest_rect.translate(offset);
   }

   pub fn get_lives(&self) -> u8 {
      self.lives
   }

   pub fn get_scale(&self) -> Vec2 {
      Vec2 { x: self.dest_rect.h, y: self.dest_rect.w }
   }

   pub fn die(&mut self) {
      self.is_alive = false;
   }

   pub fn get_frame_dimensions(&self) -> Result<Vec2, GameError> {
      self.sprite_alive.get_frame_dimensions(0)
   }

   pub fn bullet_collision_with_enemy(&self, enemy: &Enemy, image_dimensions: Vec2) -> bool {
      if self.bullet.in_air {
         let bullet_coords = self.bullet.get_coords();
         let bullet_dim = self.bullet.get_frame_dimensions().unwrap();
         let bullet_scale = self.bullet.get_scale();

         let enemy_coords = enemy.get_coords();
         let enemy_dim = enemy.get_frame_dimensions().unwrap();
         let enemy_scaling = enemy.get_scale();

         if bullet_coords.y <= enemy_coords.y + enemy_dim.x * image_dimensions.x * enemy_scaling.x &&
            bullet_coords.y * bullet_dim.x * image_dimensions.x * bullet_scale.x >= enemy_coords.y + enemy_dim.x * image_dimensions.x * enemy_scaling.x {
            if (bullet_coords.x + 10.0 >= enemy_coords.x &&
               bullet_coords.x + 10.0 <= enemy_coords.x + enemy_dim.y * image_dimensions.y * enemy_scaling.y - 30.0)
               ||
               (bullet_coords.x + bullet_dim.y * image_dimensions.y * bullet_scale.y >= enemy_coords.x &&
                bullet_coords.x + bullet_dim.y * image_dimensions.y * bullet_scale.y <= enemy_coords.x + enemy_dim.y * image_dimensions.y * enemy_scaling.y - 30.0)
            {
               return true;
            }
         }
      }

      return false;
   }

   pub fn set_bullet_in_air(&mut self, in_air: bool) {
      self.bullet.set_in_air(in_air);
   }

   pub fn lose_life(&mut self) {
      self.lives -= 1;

      if self.lives == 0 {
         self.die();
      }
   }

   pub fn revive(&mut self, position: Vec2) {
      self.is_alive = true;
      self.lives = PLAYER_NUM_LIVES;
      self.dest_rect.x = position.x;
      self.dest_rect.y = position.y;
   }
}