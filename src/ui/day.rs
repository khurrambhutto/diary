use crate::{
  app::{Diary, View},
  model::{self, words},
  theme::SERIF,
  ui::primitives::*,
};
use gpui_kit::prelude::*;
use gpui_kit::{
  base::input::{InputEditorStyle, Textarea},
  *,
};

impl Diary {
  pub fn day(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
    let wide = window.viewport_size().width >= px(1100.);
    let p = self.palette;
    let past = self.date != self.current_day;
    let days = (self.current_day - self.date).num_days();
    let eyebrow = if past {
      if days == 1 {
        "Looking back · yesterday".into()
      } else {
        format!("Looking back · {days} days ago")
      }
    } else if self.data.journal.is_empty() && self.data.habits.habits.is_empty() {
      "Welcome to Diary".into()
    } else {
      String::new()
    };
    let head = div()
      .flex()
      .flex_col()
      .child(div().h(px(20.)).text_color(p.soft).child(eyebrow))
      .w_full()
      .when(wide, |el| el.max_w(px(640.)))
      .child(
        div()
          .flex()
          .items_baseline()
          .justify_between()
          .gap(px(16.))
          .mt(px(2.))
          .child(heading(self.date.format("%A, %B %-d").to_string()))
          .when(past, |el| {
            el.child(text_button("jump-today", "Jump to today", p).on_click(
              cx.listener(|this, _, window, cx| this.select_day(this.current_day, window, cx)),
            ))
          }),
      );
    let habits = self.habits(window, cx);
    let todos = self.todos(window, cx);
    let cards = div()
      .flex()
      .gap(px(16.))
      .when(wide, |el| el.flex_col().w(px(300.)).flex_none().mt(px(22.)))
      .when(!wide, |el| el.w_full().mt(px(24.)))
      .child(div().flex_1().min_w_0().child(habits))
      .child(div().flex_1().min_w_0().child(todos));
    let max_rows = self
      .data
      .habits
      .habits
      .len()
      .max(self.data.todos.len())
      .max(1);
    let cards_height = 14. + 26. + max_rows as f32 * 34. + 34. + 8. + 2.;
    let note_height = (f32::from(window.viewport_size().height)
      - if wide { 257. } else { 289. + cards_height })
    .max(160.);
    let count = words(self.data.note(self.date));
    let meta = if past {
      format!("Read-only · {count} words")
    } else if count > 0 {
      format!("{count} words")
    } else {
      String::new()
    };
    self.note.update(cx, |state, cx| {
      state.set_readonly(past || model::today() != self.date, cx);
      state.set_editor_style(InputEditorStyle {
        foreground: if past { p.soft } else { p.ink },
        muted_foreground: p.faint,
        background: p.paper,
        selection: p.tint,
        caret: p.strong,
        ..Default::default()
      });
      state.set_editor_paddings(Edges::default());
    });
    let note = div()
      .w_full()
      .when(wide, |el| el.max_w(px(640.)))
      .flex()
      .flex_col()
      .mt(px(if wide { 24. } else { 32. }))
      .child(
        div()
          .border_b_1()
          .border_color(p.line)
          .pb(px(10.))
          .child(header("Daily Note", meta, p)),
      )
      .child(
        div()
          .h(px(note_height))
          .mt(px(16.))
          .font_family(SERIF)
          .text_size(px(17.))
          .line_height(px(28.9))
          .child(Textarea::new(&self.note)),
      );
    let column = div()
      .flex()
      .flex_col()
      .flex_1()
      .min_w_0()
      .when(wide, |el| el.items_center())
      .child(head);
    if wide {
      div()
        .flex()
        .items_start()
        .gap(px(56.))
        .w_full()
        .child(column.child(note))
        .child(cards)
        .into_any_element()
    } else {
      column.child(cards).child(note).into_any_element()
    }
  }
}
impl Render for Diary {
  fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    let p = self.palette;
    let wide = window.viewport_size().width >= px(1100.) && self.view == View::Day;
    let mut root = div()
      .id("diary")
      .key_context("Diary")
      .track_focus(&self.focus)
      .on_action(cx.listener(Self::go_to_today))
      .on_action(cx.listener(Self::previous_day))
      .on_action(cx.listener(Self::next_day))
      .on_action(cx.listener(Self::save_now))
      .on_action(cx.listener(Self::dismiss))
      .relative()
      .size_full()
      .flex()
      .flex_col()
      .bg(p.paper)
      .rounded(crate::window_frame::corner_radius(window))
      .overflow_hidden()
      .text_color(p.ink)
      .font_family(crate::theme::SANS)
      .text_size(px(14.))
      .line_height(px(19.6))
      .child(self.titlebar(window, cx))
      .child(self.topbar(cx));
    if let Some(error) = &self.load_error {
      return root
        .child(
          div()
            .p(px(40.))
            .child(heading("Your notebook couldn't open"))
            .child(div().mt(px(16.)).text_color(p.soft).child(error.clone())),
        )
        .into_any_element();
    }
    if let Some(error) = &self.save_error {
      root = root.child(
        div()
          .px(px(40.))
          .py(px(8.))
          .bg(p.tint)
          .child(error.clone())
          .child(
            text_button("retry-save", "Try saving again", p).on_click(cx.listener(
              |this, _, _, cx| {
                this.save_error = this.flush().err();
                cx.notify();
              },
            )),
          ),
      );
    }
    let content = match self.view {
      View::Day => self.day(window, cx),
      View::Analytics => self.analytics(cx).into_any_element(),
      View::Settings => self.settings(cx).into_any_element(),
    };
    root = root.child(
      div()
        .id("page-scroll")
        .flex_1()
        .min_h_0()
        .overflow_y_scroll()
        .child(
          div()
            .mx_auto()
            .max_w(px(if wide { 1240. } else { 720. }))
            .w_full()
            .px(px(40.))
            .pt(px(12.))
            .pb(px(32.))
            .child(content)
            .with_animation(
              ("page-enter", self.nav_epoch),
              Animation::new(std::time::Duration::from_millis(220))
                .with_easing(gpui_kit::component::animation::ease_out_cubic),
              |el, delta| el.opacity(0.25 + delta * 0.75),
            ),
        ),
    );
    if self.calendar.is_some() {
      root = root.child(self.calendar_overlay(cx));
    }
    if self.menu.is_some() {
      root = root.child(self.habit_menu(cx));
    }
    root.into_any_element()
  }
}
