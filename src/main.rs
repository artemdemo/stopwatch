mod texture_utils;

use notan::draw::*;
use notan::prelude::*;
use texture_utils::*;

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
  colon_texture: Texture,
  avg_num_texture_width: f32,
}

fn setup(gfx: &mut Graphics) -> State {
  let clear_options = ClearOptions::color(Color::new(0.4, 0.4, 0.4, 1.0));

  let num_textures = load_num_textures(gfx);
  let num_textures_len = num_textures.len();
  let mut total_width: f32 = 0.0;
  for texture in &num_textures {
    total_width += texture.width();
  }

  State {
    clear_options,
    num_textures,
    colon_texture: gfx
      .create_texture()
      .from_image(include_bytes!("assets/colon-0.png"))
      .build()
      .unwrap(),
    avg_num_texture_width: total_width / num_textures_len as f32,
  }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
  let mut renderer = gfx.create_renderer();

  renderer.begin(Some(state.clear_options));
  renderer.end();

  gfx.render(&renderer);

  let start = SystemTime::now();
  let duration = start.duration_since(UNIX_EPOCH).unwrap_or_default();

  create_time_renderer(
    gfx,
    state,
    duration.as_secs(),
    (W_WIDTH / 2) as f32,
    (W_HEIGHT / 2) as f32,
  );
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

  let (h, m, s) = convert_seconds(seconds);
  let mut parts: Vec<usize> = vec![];

  let (first, second) = split_number(h);
  parts.push(first);
  parts.push(second);

  parts.push(COLON_NUM); // colon ":"

  let (first, second) = split_number(m);
  parts.push(first);
  parts.push(second);

  parts.push(COLON_NUM); // colon ":"

  let (first, second) = split_number(s);
  parts.push(first);
  parts.push(second);

  // 00:00:00 - 8 characters in total
  // 6 of them are numbers
  // + 1 avg character width for 2 colons
  let total_width: f32 = state.avg_num_texture_width * 6.0 + state.avg_num_texture_width;

  let mut cursor_x = x / SCALE - total_width / 2.0 + state.avg_num_texture_width / 2.0;

  // ToDo: We don't have to calculate height of the texure at every render.
  // It is not going to change. We can calculate it right after loading all texturese in setup.
  let cursor_y = y / SCALE - get_texture_from_state(state, 0).height() / 2.0;

  for part in &parts {
    let texture = get_texture_from_state(state, *part);
    let pos_x = if *part == COLON_NUM {
      cursor_x - texture.width() / 1.3
    } else {
      cursor_x - texture.width() / 2.0
    };
    let pos_y = cursor_y;
    draw
      .image(texture)
      .position(pos_x, pos_y)
      .scale(SCALE, SCALE);

    // draw.circle(5.).position(pos_x * SCALE, pos_y * SCALE).color(Color::new(0.5, 0.0, 0.0, 1.0));
    // draw.circle(5.).position(cursor_x * SCALE, pos_y * SCALE).color(Color::new(0.0, 0.7, 0.0, 1.0));

    cursor_x += if *part == COLON_NUM {
      state.avg_num_texture_width * 0.5
    } else {
      state.avg_num_texture_width
    };
  }

  gfx.render(&draw);
}
