mod load_textures;

use load_textures::load_textures;
use notan::draw::*;
use notan::prelude::*;

use std::time::{SystemTime, UNIX_EPOCH};

const W_WIDTH: u32 = 700;
const W_HEIGHT: u32 = 300;
const SCALE: f32 = 0.2;

#[notan_main]
fn main() -> Result<(), String> {
  notan::init_with(setup)
    .add_config(
      WindowConfig::default()
        .set_size(W_WIDTH, W_HEIGHT)
        .set_title("Stopwatch"),
    )
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

  let start = SystemTime::now();
  let duration = start.duration_since(UNIX_EPOCH).unwrap_or_default();

  create_time_renderer(gfx, state, duration.as_secs(), (W_WIDTH / 2) as f32, (W_HEIGHT / 2) as f32);
}

fn convert_seconds(total_seconds: u64) -> (u64, u64, u64) {
  let seconds_in_a_day = 24 * 60 * 60;
  let seconds_today = total_seconds % seconds_in_a_day;

  let hours = seconds_today / 3600;
  let minutes = (seconds_today % 3600) / 60;
  let seconds = seconds_today % 60;

  (hours, minutes, seconds)
}

fn split_number(num: u64) -> (usize, usize) {
  let first = (num / 10) as usize;
  let second = num as usize - first * 10;
  (first, second)
}

fn create_time_renderer(gfx: &mut Graphics, state: &mut State, seconds: u64, x: f32, y: f32) {
  let mut draw = gfx.create_draw();

  // ToDo: Merge seconds calculation into one method.
  // This split (`convert_seconds`, `split_number`, etc) doesn't make much sense now.
  let (h, m, s) = convert_seconds(seconds);
  let mut parts: Vec<usize> = vec![];

  let (first, second) = split_number(h);
  parts.push(first);
  parts.push(second);

  let (first, second) = split_number(m);
  parts.push(first);
  parts.push(second);

  let (first, second) = split_number(s);
  parts.push(first);
  parts.push(second);

  let mut total_width: f32 = 0.0;

  for part in &parts {
    total_width += state.num_textures[*part].width();
  }

  let mut cursor_x = x / SCALE - total_width / 2.0;

  for part in &parts {
    let num_texture = &state.num_textures[*part];
    draw
      .image(num_texture)
      .position(cursor_x, 0.0)
      .scale(SCALE, SCALE);

    cursor_x += num_texture.width();
  }

  gfx.render(&draw);
}
