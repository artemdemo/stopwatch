use notan::log::warn;

#[derive(Debug, PartialEq, Clone)]
pub enum ColorTheme {
  Light,
  Dark,
}

pub fn get_os_theme() -> ColorTheme {
  let result = dark_light::detect();
  if let Ok(mode) = result {
    return match mode {
      dark_light::Mode::Light => ColorTheme::Light,
      dark_light::Mode::Dark => ColorTheme::Dark,
      dark_light::Mode::Unspecified => ColorTheme::Dark,
    };
  } else {
    warn!("Error occured while getting theme");
  }
  return ColorTheme::Dark;
}
