use ggez::Context;
use ggez::input::keyboard::{KeyCode};

pub fn update_shoot(ctx: &Context) -> bool {
   let k_ctx = &ctx.keyboard;

   k_ctx.is_key_just_pressed(KeyCode::Space)
}

pub fn is_space_pressed(ctx: &Context) -> bool {
   let k_ctx = &ctx.keyboard;

   k_ctx.is_key_just_pressed(KeyCode::Space)
}

pub fn update_movement(ctx: &Context) -> i8 {
   let k_ctx = &ctx.keyboard;

   if k_ctx.is_key_pressed(KeyCode::Left) {
      -1
   } else if k_ctx.is_key_pressed(KeyCode::Right) {
      1
   } else {
      0
   }
}