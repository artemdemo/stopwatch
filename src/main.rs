use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
  notan::init_with(setup)
    .add_config(WindowConfig::default().set_size(700, 300))
    .draw(draw)
    .build()
}

#[derive(AppState)]
struct State {
  clear_options: ClearOptions,
}

fn setup(gfx: &mut Graphics) -> State {
  let clear_options = ClearOptions::color(Color::new(0.3, 0.3, 0.3, 1.0));
  State { clear_options }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
  let mut renderer = gfx.create_renderer();

  renderer.begin(Some(state.clear_options));
  renderer.end();

  gfx.render(&renderer);
}
