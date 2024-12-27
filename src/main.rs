mod load_textures;

use load_textures::load_textures;
use notan::draw::*;
use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
  notan::init_with(setup)
    .add_config(WindowConfig::default().set_size(700, 300))
    .add_config(DrawConfig)
    .draw(draw)
    .build()
}

#[derive(AppState)]
struct State {
  clear_options: ClearOptions,
  num_textures: [Texture; 10],
}

fn setup(gfx: &mut Graphics) -> State {
  let clear_options = ClearOptions::color(Color::new(0.4, 0.4, 0.4, 1.0));

  State {
    clear_options,
    num_textures: load_textures(gfx),
  }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
  let mut renderer = gfx.create_renderer();

  renderer.begin(Some(state.clear_options));
  renderer.end();

  gfx.render(&renderer);

  let mut draw = gfx.create_draw();
  draw.image(&state.num_textures[2]).position(0.0, 0.0).scale(0.2, 0.2);
  gfx.render(&draw);
}
