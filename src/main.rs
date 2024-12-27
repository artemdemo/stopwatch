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
  img: Texture,
}

fn setup(gfx: &mut Graphics) -> State {
  let clear_options = ClearOptions::color(Color::new(0.4, 0.4, 0.4, 1.0));
  let img = gfx
    .create_texture()
    .from_image(include_bytes!("assets/num-0-0.png"))
    .build()
    .unwrap();
  State { clear_options, img }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
  let mut renderer = gfx.create_renderer();

  renderer.begin(Some(state.clear_options));
  renderer.end();

  gfx.render(&renderer);

  let mut draw = gfx.create_draw();
  draw.image(&state.img).position(0.0, 0.0).scale(0.2, 0.2);
  gfx.render(&draw);
}
