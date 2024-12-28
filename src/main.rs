mod texture_utils;

use notan::draw::*;
use notan::prelude::*;
use texture_utils::*;

use std::time::{SystemTime, UNIX_EPOCH};

const W_WIDTH: u32 = 900;
const W_HEIGHT: u32 = 300;

#[notan_main]
fn main() -> Result<(), String> {
  notan::init_with(setup)
    .add_config(
      WindowConfig::default()
        .set_size(W_WIDTH, W_HEIGHT)
        .set_title("Stopwatch")
        .set_resizable(true),
    )
    .add_config(DrawConfig)
    .draw(draw)
    .build()
}

#[derive(AppState)]
struct State {
  clear_options: ClearOptions,
  num_textures: [Texture; 10],
  colon_texture: [Texture; 3],
  avg_num_texture_width: f32,
  prev_render_timestamp: u128,
}

fn calc_scale(w_height: u32) -> f32 {
  w_height as f32 / 1100.0
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
    colon_texture: load_colon_textures(gfx),
    avg_num_texture_width: total_width / num_textures_len as f32,
    prev_render_timestamp: SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap_or_default()
      .as_millis(),
  }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
  let duration = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default();

  let mills = duration.as_millis();
  let delta = mills - state.prev_render_timestamp;

  if delta > 100 {
    state.prev_render_timestamp = mills;
    let time_parts = create_time_parts( duration.as_secs());
    create_time_renderer(gfx, state, time_parts);
  }
}

fn split_number(num: u64) -> (usize, usize) {
  let first = (num / 10) as usize;
  let second = num as usize - first * 10;
  (first, second)
}

fn create_time_parts(seconds: u64) -> Vec<usize> {
  let seconds_in_a_day = 24 * 60 * 60;
  let seconds_today = seconds % seconds_in_a_day;

  let hours = seconds_today / 3600;
  let minutes = (seconds_today % 3600) / 60;
  let seconds = seconds_today % 60;

  let mut parts: Vec<usize> = vec![];

  let (first, second) = split_number(hours);
  parts.push(first);
  parts.push(second);

  parts.push(COLON_NUM); // colon ":"

  let (first, second) = split_number(minutes);
  parts.push(first);
  parts.push(second);

  parts.push(COLON_NUM); // colon ":"

  let (first, second) = split_number(seconds);
  parts.push(first);
  parts.push(second);

  return parts;
}

fn create_time_renderer(gfx: &mut Graphics, state: &mut State, time_parts: Vec<usize>) {
  let (w_width, w_height) = gfx.size();
  let mut draw = gfx.create_draw();
  draw.clear(Color::GRAY);

  let scale = calc_scale(w_height);
  let center_x = w_width as f32 / 2.0;
  let center_y = w_height as f32 / 2.0;

  // 00:00:00 - 8 characters in total
  // 6 of them are numbers
  // + 1 avg character width for 2 colons
  let total_width: f32 = state.avg_num_texture_width * 6.0 + state.avg_num_texture_width;

  let mut cursor_x = center_x / scale - total_width / 2.0 + state.avg_num_texture_width / 2.0;

  // ToDo: We don't have to calculate height of the texure at every render.
  // It is not going to change. We can calculate it right after loading all texturese in setup.
  let cursor_y = center_y / scale - get_texture_from_state(state, 0).height() / 2.0;

  let mut nums: Vec<usize> = vec![];

  for part in &time_parts {
    let texture = get_texture_from_state(state, *part);
    nums.push(*part);
    let pos_x = if *part == COLON_NUM {
      cursor_x - texture.width() / 1.3
    } else {
      cursor_x - texture.width() / 2.0
    };
    let pos_y = cursor_y;
    draw
      .image(texture)
      .position(pos_x, pos_y)
      .scale(scale, scale);

    cursor_x += if *part == COLON_NUM {
      state.avg_num_texture_width * 0.5
    } else {
      state.avg_num_texture_width
    };
  }

  println!("{:?}", nums);

  gfx.render(&draw);
}
