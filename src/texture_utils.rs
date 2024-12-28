use notan::prelude::*;

use crate::State;

pub const COLON_NUM: usize = 10;

pub fn load_num_textures(gfx: &mut Graphics) -> [Texture; 10] {
  let num_textures: [Texture; 10] = [
    gfx
      .create_texture()
      .from_image(include_bytes!("assets/num-0-0.png"))
      .build()
      .unwrap(),
    gfx
      .create_texture()
      .from_image(include_bytes!("assets/num-1-0.png"))
      .build()
      .unwrap(),
    gfx
      .create_texture()
      .from_image(include_bytes!("assets/num-2-0.png"))
      .build()
      .unwrap(),
    gfx
      .create_texture()
      .from_image(include_bytes!("assets/num-3-0.png"))
      .build()
      .unwrap(),
    gfx
      .create_texture()
      .from_image(include_bytes!("assets/num-4-0.png"))
      .build()
      .unwrap(),
    gfx
      .create_texture()
      .from_image(include_bytes!("assets/num-5-0.png"))
      .build()
      .unwrap(),
    gfx
      .create_texture()
      .from_image(include_bytes!("assets/num-6-0.png"))
      .build()
      .unwrap(),
    gfx
      .create_texture()
      .from_image(include_bytes!("assets/num-7-0.png"))
      .build()
      .unwrap(),
    gfx
      .create_texture()
      .from_image(include_bytes!("assets/num-8-0.png"))
      .build()
      .unwrap(),
    gfx
      .create_texture()
      .from_image(include_bytes!("assets/num-9-0.png"))
      .build()
      .unwrap(),
  ];

  num_textures
}

pub fn get_texture_from_state(state: &State, part: usize) -> &Texture {
  let texture = if part == COLON_NUM {
    &state.colon_texture
  } else {
    &state.num_textures[part]
  };
  texture
}
