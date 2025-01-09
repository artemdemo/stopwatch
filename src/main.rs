mod texture_utils;

// I started using `std:time`, but it can't be compiled into WASM
// So I switched to `chrono`
use chrono::Utc;

use notan::draw::*;
use notan::prelude::*;
use rand::seq::SliceRandom;
use texture_utils::*;

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

#[derive(Debug, PartialEq)]
enum StopwatchDirection {
  None,
  Up,
  Down,
}

enum TimeState {
  Time,
  Stopwatch {
    paused: bool,
    direction: StopwatchDirection,
  },
}

//language=glsl
const FRAGMENT: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) in vec2 v_uvs;
    layout(location = 1) in vec4 v_color;

    layout(binding = 0) uniform sampler2D u_texture;
    layout(set = 0, binding = 1) uniform TextureInfo {
      vec3 u_color;
    };

    layout(location = 0) out vec4 color;

    void main() {
      vec4 texColor = texture(u_texture, v_uvs);
      color = vec4(u_color, texColor.a);
    }
"#
};

#[derive(AppState)]
struct State {
  num_textures: [Texture; 30],
  colon_textures: [Texture; 3],
  avg_num_texture_width: f32,
  texture_height: f32,
  prev_render_timestamp: i64,
  draw: Draw,
  timer_last_addition: i64,
  timer_secs: i64,
  pipeline: Pipeline,
  uniforms: Option<Buffer>,
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

  let pipeline = create_image_pipeline(gfx, Some(&FRAGMENT)).unwrap();

  State {
    num_textures,
    pipeline,
    uniforms: None,
    colon_textures: load_colon_textures(gfx),
    avg_num_texture_width: total_width / num_textures_len as f32,
    texture_height,
    prev_render_timestamp: Utc::now().timestamp_millis(),
    draw: gfx.create_draw(),
    timer_last_addition: Utc::now().timestamp_millis(),
    timer_secs: 0,
    time_state: TimeState::Time,
  }
}

fn reset_stopwatch(state: &mut State) {
  state.timer_secs = 0;
  state.timer_last_addition = Utc::now().timestamp_millis();
  state.time_state = TimeState::Stopwatch {
    paused: true,
    direction: StopwatchDirection::None,
  };
}

fn is_dark_theme() -> bool {
  let mode = dark_light::detect();
  match mode {
    dark_light::Mode::Light => false,
    dark_light::Mode::Dark => true,
    dark_light::Mode::Default => true,
  }
}

fn update(app: &mut App, state: &mut State) {
  match &mut state.time_state {
    TimeState::Stopwatch { paused, direction } => {
      if *paused == true {
        let mut seconds = 0;
        if app.keyboard.was_released(KeyCode::Key1) {
          seconds = 1;
        }
        if app.keyboard.was_released(KeyCode::Key2) {
          seconds = 2;
        }
        if app.keyboard.was_released(KeyCode::Key3) {
          seconds = 3;
        }
        if app.keyboard.was_released(KeyCode::Key4) {
          seconds = 4;
        }
        if app.keyboard.was_released(KeyCode::Key5) {
          seconds = 5;
        }
        if app.keyboard.was_released(KeyCode::Key6) {
          seconds = 6;
        }
        if app.keyboard.was_released(KeyCode::Key7) {
          seconds = 7;
        }
        if app.keyboard.was_released(KeyCode::Key8) {
          seconds = 8;
        }
        if app.keyboard.was_released(KeyCode::Key9) {
          seconds = 9;
        }
        if app.keyboard.was_released(KeyCode::Key0) {
          state.timer_secs = state.timer_secs * 10;
        }
        if seconds > 0 {
          state.timer_secs = state.timer_secs * 10 + seconds * 60;
        }
      }
      if app.keyboard.was_released(KeyCode::S) {
        *paused = !*paused;
        if *paused == false {
          state.timer_last_addition = Utc::now().timestamp_millis();
          if *direction == StopwatchDirection::None {
            if state.timer_secs > 0 {
              *direction = StopwatchDirection::Down;
            } else {
              *direction = StopwatchDirection::Up;
            }
          }
        }
      }
      if app.keyboard.was_released(KeyCode::R) {
        // Reset stopwatch
        reset_stopwatch(state);
      }
      if app.keyboard.was_released(KeyCode::T) {
        // Switch back to regular time
        state.time_state = TimeState::Time;
        state.timer_secs = 0;
      }
    }
    TimeState::Time => {
      if app.keyboard.was_released(KeyCode::S) {
        state.time_state = TimeState::Stopwatch {
          direction: StopwatchDirection::None,
          paused: true,
        };
      }
    }
  }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
  let system_time: i64 = Utc::now().timestamp_millis();

  match &mut state.time_state {
    TimeState::Time => {}
    TimeState::Stopwatch { paused, direction } => {
      if *paused == false {
        let timer_diff = (system_time - state.timer_last_addition) / 1000;
        if timer_diff >= 1 {
          match *direction {
            StopwatchDirection::Up => {
              state.timer_secs = state.timer_secs + timer_diff;
            }
            StopwatchDirection::Down => {
              if state.timer_secs >= timer_diff {
                state.timer_secs = state.timer_secs - timer_diff;
              } else {
                reset_stopwatch(state);
              }
            }
            StopwatchDirection::None => {
              panic!("This shouldn't happen, but here we are");
            }
          }
          state.timer_last_addition = system_time;
        }
      }
    }
  }

  // state.timer_unpaused;
  let duration = match &state.time_state {
    TimeState::Time => system_time / 1000,
    TimeState::Stopwatch {
      paused: _,
      direction: _,
    } => state.timer_secs,
  };

  let system_time_mills = system_time;
  let delta = system_time_mills - state.prev_render_timestamp;

  if delta > 50 {
    state.draw = gfx.create_draw();

    if is_dark_theme() {
      state.uniforms = Some(gfx
        .create_uniform_buffer(1, "TextureInfo")
        .with_data(&[Color::WHITE.rgb()])
        .build()
        .unwrap());
      state.draw.clear(Color::new(0.25, 0.25, 0.25, 1.0));
    } else {
      state.draw.clear(Color::GRAY);
    }

    let (w_width, w_height) = gfx.size();
    state.prev_render_timestamp = system_time_mills;
    let time_parts = create_time_parts(duration);

    {
      let mut pipeline = state.draw.image_pipeline();
      if let Some (uniforms) = &state.uniforms {
        pipeline.pipeline(&state.pipeline);
        pipeline.uniform_buffer(&uniforms);
      }
    }

    apply_num_textures(state, time_parts, w_width, w_height);

    state.draw.image_pipeline().remove();
  }

  gfx.render(&state.draw);
}

fn split_number(num: i64) -> (usize, usize) {
  let first = (num / 10) as usize;
  let second = num as usize - first * 10;
  (first, second)
}

fn create_time_parts(seconds: i64) -> Vec<usize> {
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
