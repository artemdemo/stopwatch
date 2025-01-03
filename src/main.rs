mod texture_utils;

use notan::draw::*;
use notan::prelude::*;
use rand::seq::SliceRandom;
use texture_utils::*;

use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

const W_WIDTH: u32 = 900;
const W_HEIGHT: u32 = 300;
const SCALE_FACTOR: f32 = 600.0;

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
    .update(update)
    .draw(draw)
    .build()
}

#[derive(Default, Debug, Clone)]
enum StopwatchDirection {
  #[default]
  Up,
}

enum TimeState {
  Time,
  Stopwatch {
    paused: bool,
    direction: StopwatchDirection,
  },
}

#[derive(AppState)]
struct State {
  num_textures: [Texture; 30],
  colon_textures: [Texture; 3],
  avg_num_texture_width: f32,
  texture_height: f32,
  prev_render_timestamp: u128,
  draw: Draw,
  timer_started: Duration,
  duration: Duration,
  time_state: TimeState,
}

fn setup(gfx: &mut Graphics) -> State {
  let num_textures = load_num_textures(gfx);
  let num_textures_len = num_textures.len();
  let texture_height = num_textures[0].height();
  let mut total_width: f32 = 0.0;
  for texture in &num_textures {
    total_width += texture.width();
  }

  State {
    num_textures,
    colon_textures: load_colon_textures(gfx),
    avg_num_texture_width: total_width / num_textures_len as f32,
    texture_height,
    prev_render_timestamp: SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap_or_default()
      .as_millis(),
    draw: gfx.create_draw(),
    timer_started: Duration::new(0, 0),
    duration: Duration::new(0, 0),
    time_state: TimeState::Time,
  }
}

fn update(app: &mut App, state: &mut State) {
  match &mut state.time_state {
    TimeState::Stopwatch {
      paused,
      direction: _,
    } => {
      if app.keyboard.was_released(KeyCode::S) {
        println!("S");
        state.timer_started = SystemTime::now()
          .duration_since(UNIX_EPOCH)
          .unwrap_or_default();
        *paused = !*paused;
      }
      if app.keyboard.was_released(KeyCode::R) {
        // Reset stopwatch
        println!("R");
        state.duration = Duration::new(0, 0);
      }
      if app.keyboard.was_released(KeyCode::T) {
        // Switch back to regular time
        println!("T");
        state.time_state = TimeState::Time;
        state.duration = Duration::new(0, 0);
      }
    }
    TimeState::Time => {
      if app.keyboard.was_released(KeyCode::S) {
        println!("S");
        state.time_state = TimeState::Stopwatch {
          direction: StopwatchDirection::Up,
          paused: true,
        };
      }
    }
  }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
  let system_time = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default();

  // state.timer_started;
  let duration = match &state.time_state {
    TimeState::Time => system_time,
    TimeState::Stopwatch { paused, direction } => system_time.checked_sub(state.timer_started).unwrap_or_default(),
  };

  let system_time_mills = system_time.as_millis();
  let delta = system_time_mills - state.prev_render_timestamp;

  if delta > 50 {
    state.draw = gfx.create_draw();
    state.draw.clear(Color::GRAY);

    let (w_width, w_height) = gfx.size();
    state.prev_render_timestamp = system_time_mills;
    let time_parts = create_time_parts(duration.as_secs());
    apply_num_textures(state, time_parts, w_width, w_height);
  }

  gfx.render(&state.draw);
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

fn apply_num_textures(state: &mut State, time_parts: Vec<usize>, w_width: u32, w_height: u32) {
  let center_x = w_width as f32 / 2.0;
  let center_y = w_height as f32 / 2.0;

  // 00:00:00 - 8 characters in total
  // 6 of them are numbers
  // + 1 avg character width for 2 colons
  let total_width: f32 = state.avg_num_texture_width * 6.0 + state.avg_num_texture_width;

  let texture_ratio = state.texture_height / total_width;
  let window_ratio = w_height as f32 / w_width as f32;

  let scale = if texture_ratio < window_ratio {
    texture_ratio * w_width as f32 / SCALE_FACTOR
  } else {
    texture_ratio * w_height as f32 / (SCALE_FACTOR * texture_ratio)
  };

  let mut cursor_x = center_x / scale - total_width / 2.0 + state.avg_num_texture_width / 2.0;

  let cursor_y = center_y / scale - &state.texture_height / 2.0;

  let mut nums: Vec<usize> = vec![];

  for part in &time_parts {
    let texture = if *part == COLON_NUM {
      &state
        .colon_textures
        .choose(&mut rand::thread_rng())
        .unwrap_or(&state.colon_textures[0])
    } else {
      let variants: [usize; 3] = [0, 1, 2];
      let num_variant = *part * 3 + variants.choose(&mut rand::thread_rng()).unwrap_or(&0);
      &state.num_textures[num_variant]
    };

    nums.push(*part);

    let pos_x = if *part == COLON_NUM {
      cursor_x - texture.width() / 1.3
    } else {
      cursor_x - texture.width() / 2.0
    };
    let pos_y = cursor_y;

    state
      .draw
      .image(texture)
      .position(pos_x, pos_y)
      .scale(scale, scale);

    cursor_x += if *part == COLON_NUM {
      state.avg_num_texture_width * 0.5
    } else {
      state.avg_num_texture_width
    };
  }
}
