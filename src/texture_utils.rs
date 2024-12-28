use notan::prelude::*;

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

pub fn load_colon_textures(gfx: &mut Graphics) -> [Texture; 3] {
  let colon_textures: [Texture; 3] = [
    gfx
      .create_texture()
      .from_image(include_bytes!("assets/colon-0.png"))
      .build()
      .unwrap(),
    gfx
      .create_texture()
      .from_image(include_bytes!("assets/colon-1.png"))
      .build()
      .unwrap(),
    gfx
      .create_texture()
      .from_image(include_bytes!("assets/colon-2.png"))
      .build()
      .unwrap(),
  ];
  colon_textures
}
