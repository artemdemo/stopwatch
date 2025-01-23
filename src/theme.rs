use notan::log::warn;

#[derive(Debug, PartialEq, Clone)]
pub enum ColorTheme {
  Light,
  Dark,
}

pub fn get_os_theme() -> ColorTheme {
  let result = dark_light::detect();
  match result {
    Ok(mode) => {
      return match mode {
        dark_light::Mode::Light => ColorTheme::Light,
        dark_light::Mode::Dark => ColorTheme::Dark,
        dark_light::Mode::Unspecified => ColorTheme::Dark,
      };
    },
    Err(err) => {
      warn!("Error occured while getting theme:");
      dbg!(err);
    },
  }
  return ColorTheme::Dark;
}
