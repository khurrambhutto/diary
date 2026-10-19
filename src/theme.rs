use gpui_kit::{
  base::{Theme, ThemeAppearance},
  *,
};

pub const SANS: &str = "Atkinson Hyperlegible";
pub const SERIF: &str = "Newsreader 16pt";
#[derive(Clone, Copy)]
pub struct Palette {
  pub paper: Hsla,
  pub raised: Hsla,
  pub subtle: Hsla,
  pub ink: Hsla,
  pub soft: Hsla,
  pub faint: Hsla,
  pub accent: Hsla,
  pub strong: Hsla,
  pub tint: Hsla,
  pub on_accent: Hsla,
  pub line: Hsla,
  pub track: Hsla,
  pub heat: [Hsla; 5],
}
impl Palette {
  pub fn new(dark: bool) -> Self {
    let c = |v| Hsla::from(rgb(v));
    let v = if dark {
      [
        0x1B1916, 0x23201C, 0x2A2622, 0xECE5DA, 0xB3AA9D, 0x7C7469, 0xA499CF, 0xC3B8EA, 0x2F2A3B,
        0x1B1916, 0x34302A, 0x2C2824, 0x383149, 0x524870, 0x7A6DA6, 0xA499CF,
      ]
    } else {
      [
        0xFBF8F3, 0xFFFDF9, 0xF3EEE6, 0x2B2722, 0x5E574E, 0x8F867B, 0x8A7DB3, 0x6B5C9E, 0xECE6F3,
        0xFFFFFF, 0xE6DFD4, 0xEDE7DD, 0xDDD5EA, 0xBFB3DA, 0x9C8EC7, 0x6B5C9E,
      ]
    };
    Self {
      paper: c(v[0]),
      raised: c(v[1]),
      subtle: c(v[2]),
      ink: c(v[3]),
      soft: c(v[4]),
      faint: c(v[5]),
      accent: c(v[6]),
      strong: c(v[7]),
      tint: c(v[8]),
      on_accent: c(v[9]),
      line: c(v[10]),
      track: c(v[11]),
      heat: [c(v[11]), c(v[12]), c(v[13]), c(v[14]), c(v[15])],
    }
  }
}
pub fn apply(window: &mut Window, cx: &mut App) -> Palette {
  let dark = matches!(
    window.appearance(),
    WindowAppearance::Dark | WindowAppearance::VibrantDark
  );
  let p = Palette::new(dark);
  window.set_rem_size(px(14.));
  let theme = Theme::global_mut(cx);
  theme.appearance = if dark {
    ThemeAppearance::Dark
  } else {
    ThemeAppearance::Light
  };
  let colors = &mut theme.tokens.colors;
  colors.background = p.paper;
  colors.foreground = p.ink;
  colors.surface = p.raised;
  colors.surface_foreground = p.ink;
  colors.muted_foreground = p.faint;
  colors.border = p.line;
  colors.accent = p.accent;
  colors.accent_foreground = p.on_accent;
  colors.selection = p.tint;
  colors.ring = p.accent;
  cx.refresh_windows();
  p
}
