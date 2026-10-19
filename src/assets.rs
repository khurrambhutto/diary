use gpui_kit::*;
use std::borrow::Cow;

pub struct Assets;
macro_rules! icons {
  ($($name:literal),*) => { fn icon_data(path: &str) -> Option<&'static [u8]> {
    match path { $(concat!("diary/",$name,".svg") => Some(include_bytes!(concat!("../assets/icons/",$name,".svg"))),)* _ => None }
  } };
}
icons!(
  "calendar",
  "check",
  "chevron-left",
  "chevron-right",
  "dots",
  "gear",
  "plus",
  "win-close",
  "win-maximize",
  "win-minimize",
  "win-restore"
);
impl AssetSource for Assets {
  fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
    if let Some(bytes) = icon_data(path) {
      return Ok(Some(Cow::Borrowed(bytes)));
    }
    gpui_kit::assets::Assets.load(path)
  }
  fn list(&self, path: &str) -> Result<Vec<SharedString>> {
    gpui_kit::assets::Assets.list(path)
  }
}
pub fn register_fonts(cx: &mut App) -> Result<()> {
  cx.text_system().add_fonts(vec![
    Cow::Borrowed(include_bytes!(
      "../assets/fonts/AtkinsonHyperlegible-Regular.ttf"
    )),
    Cow::Borrowed(include_bytes!(
      "../assets/fonts/AtkinsonHyperlegible-Bold.ttf"
    )),
    Cow::Borrowed(include_bytes!("../assets/fonts/Newsreader16pt-Regular.ttf")),
    Cow::Borrowed(include_bytes!("../assets/fonts/Newsreader16pt-Medium.ttf")),
    Cow::Borrowed(include_bytes!("../assets/fonts/Newsreader16pt-Italic.ttf")),
  ])
}
