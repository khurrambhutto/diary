use crate::{
  app::{Diary, View},
  ui::primitives::*,
};
use chrono::Days;
use gpui_kit::prelude::*;
use gpui_kit::*;
impl Diary {
  pub fn titlebar(&self, window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
    let p = self.palette;
    div()
      .id("titlebar")
      .h(px(36.))
      .flex_none()
      .flex()
      .items_center()
      .justify_end()
      .on_mouse_down(MouseButton::Left, |event, window, _| {
        if event.click_count == 2 {
          window.zoom_window();
        } else {
          window.start_window_move();
        }
      })
      .when(!cfg!(target_os = "macos"), |bar| {
        bar
          .child(
            button("minimize", "Minimize", p)
              .w(px(46.))
              .h_full()
              .rounded_none()
              .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
              .child(icon("win-minimize", 11., p.soft))
              .on_click(|_, window, _| window.minimize_window()),
          )
          .child(
            button("maximize", "Maximize or restore", p)
              .w(px(46.))
              .h_full()
              .rounded_none()
              .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
              .child(icon("win-maximize", 11., p.soft))
              .on_click(|_, window, _| window.zoom_window()),
          )
          .child(
            button("close", "Close", p)
              .w(px(46.))
              .h_full()
              .rounded_none()
              .rounded_tr(crate::window_frame::corner_radius(window))
              .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
              .child(icon("win-close", 11., p.soft))
              .on_click(cx.listener(|this, _, window, cx| this.close(window, cx))),
          )
      })
  }
  pub fn topbar(&self, cx: &mut Context<Self>) -> Div {
    let p = self.palette;
    let seg = self.segment();
    let mut pill = div()
      .id("view-pill")
      .track_focus(&self.pill_focus)
      .relative()
      .flex()
      .p(px(3.))
      .border_1()
      .border_color(p.line)
      .rounded_full()
      .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
        let index = this.segment().unwrap_or(1);
        let target = match event.keystroke.key.as_str() {
          "left" => index.saturating_sub(1),
          "right" => (index + 1).min(2),
          _ => return,
        };
        match target {
          0 => this.select_day(this.current_day - Days::new(1), window, cx),
          1 => this.select_day(this.current_day, window, cx),
          _ => this.navigate(View::Analytics, None, window, cx),
        }
        this.pill_focus.focus(window, cx);
        cx.stop_propagation();
      }));
    if let Some(index) = seg {
      let from = self.previous_segment as f32 * 92.;
      let to = index as f32 * 92.;
      pill = pill.child(
        div()
          .absolute()
          .top(px(3.))
          .left(px(3. + to))
          .w(px(92.))
          .h(px(28.))
          .rounded_full()
          .bg(p.tint)
          .with_animation(
            ("pill-slide", self.nav_epoch),
            Animation::new(std::time::Duration::from_millis(240))
              .with_easing(gpui_kit::component::animation::ease_out_cubic),
            move |el, t| el.left(px(3. + from + (to - from) * t)),
          ),
      );
    }
    for (index, label) in ["Yesterday", "Today", "Analytics"].into_iter().enumerate() {
      pill = pill.child(
        button(("segment", index), label, p)
          .w(px(92.))
          .h(px(28.))
          .rounded_full()
          .when(seg == Some(index), |el| {
            el.font_weight(FontWeight::BOLD).text_color(p.strong)
          })
          .child(label)
          .on_click(cx.listener(move |this, _, window, cx| {
            match index {
              0 => this.select_day(this.current_day - Days::new(1), window, cx),
              1 => this.select_day(this.current_day, window, cx),
              _ => this.navigate(View::Analytics, None, window, cx),
            }
            this.pill_focus.focus(window, cx);
          })),
      );
    }
    div()
      .h(px(48.))
      .flex_none()
      .flex()
      .items_center()
      .px(px(16.))
      .child(
        div().flex_1().child(
          icon_button("calendar", "calendar", "Choose a day", p)
            .when(self.calendar.is_some(), |el| el.bg(p.subtle))
            .on_click(cx.listener(|this, _, window, cx| {
              this.calendar = if this.calendar.is_some() {
                None
              } else {
                Some(this.date)
              };
              this.focus.focus(window, cx);
              cx.notify();
            })),
        ),
      )
      .child(pill)
      .child(
        div().flex_1().flex().justify_end().child(
          icon_button("settings", "gear", "Settings", p)
            .when(self.view == View::Settings, |el| el.bg(p.subtle))
            .on_click(cx.listener(|this, _, window, cx| {
              this.navigate(
                if this.view == View::Settings {
                  View::Day
                } else {
                  View::Settings
                },
                None,
                window,
                cx,
              );
            })),
        ),
      )
  }
}
