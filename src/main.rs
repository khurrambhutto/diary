#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod assets;
mod inputs;
mod interactions;
mod model;
mod storage;
mod theme;
mod ui;
mod updates;
mod window_frame;

use gpui_kit::*;
fn main() {
  gpui_kit::application()
    .with_assets(assets::Assets)
    .run(|cx| {
      gpui_kit::base::init(cx);
      gpui_kit::base::Root::register_plugin(cx, window_frame::WindowFrame::new);
      assets::register_fonts(cx).expect("Cannot load Diary's bundled fonts");
      cx.on_window_closed(|cx, _| {
        if cx.windows().is_empty() {
          cx.quit();
        }
      })
      .detach();
      let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
          None,
          size(px(720.), px(1000.)),
          cx,
        ))),
        window_min_size: Some(size(px(720.), px(500.))),
        app_id: Some("com.khurram.diary".into()),
        titlebar: Some(TitlebarOptions {
          title: Some("Diary".into()),
          appears_transparent: true,
          traffic_light_position: Some(point(px(14.), px(12.))),
        }),
        app_owns_titlebar_drag: true,
        window_background: WindowBackgroundAppearance::Transparent,
        window_decorations: Some(WindowDecorations::Client),
        ..Default::default()
      };
      cx.open_window(options, |window, cx| {
        let view = cx.new(|cx| app::Diary::new(window, cx));
        cx.new(|cx| gpui_kit::base::Root::new(view, window, cx))
      })
      .expect("Cannot open Diary window");
      cx.activate(true);
    });
}
