use super::text_input::text_input;
use crate::{app::Diary, model::streaks, ui::primitives::*};
use gpui_kit::prelude::*;
use gpui_kit::*;
impl Diary {
  pub fn habits(&self, window: &Window, cx: &mut Context<Self>) -> Div {
    let p = self.palette;
    let count = self
      .data
      .habits
      .habits
      .iter()
      .filter(|h| self.data.checked(&h.id, self.date))
      .count();
    let mut list = card(p).child(header(
      "Habits",
      format!("{count} of {}", self.data.habits.habits.len()),
      p,
    ));
    if self.data.habits.habits.is_empty() {
      list = list.child(
        div()
          .text_color(p.soft)
          .my(px(2.))
          .child("Small things you'd like to do each day."),
      );
    }
    for habit in &self.data.habits.habits {
      let id = habit.id.clone();
      let checked = self.data.checked(&id, self.date);
      let streak = streaks(&self.data.habit_dates(&id, self.date), self.date).current;
      let mut row = div()
        .id(SharedString::from(format!("habit-row-{id}")))
        .flex()
        .items_center()
        .gap(px(10.))
        .h(px(34.))
        .min_w_0()
        .rounded(px(6.))
        .hover(move |s| s.bg(p.subtle));
      if self.renaming.as_ref() == Some(&id) {
        row = row.child(
          text_input(&self.rename_input, "Rename habit", p, window, cx)
            .border_1()
            .border_color(p.accent)
            .rounded(px(6.))
            .px(px(8.))
            .h(px(28.)),
        );
      } else {
        let toggle_id = id.clone();
        row = row
          .child(
            checkbox(
              SharedString::from(format!("check-{id}")),
              habit.label.clone(),
              checked,
              false,
              p,
            )
            .on_change({
              let toggle = cx.listener(move |this, _: &ClickEvent, _, cx| {
                this.data.toggle_habit(&toggle_id, this.date);
                this.persist();
                cx.notify();
              });
              move |_, event, window, cx| toggle(event, window, cx)
            }),
          )
          .child(
            div()
              .id(SharedString::from(format!("habit-label-{id}")))
              .flex_1()
              .min_w_0()
              .overflow_hidden()
              .text_ellipsis()
              .whitespace_nowrap()
              .cursor_pointer()
              .child(habit.label.clone())
              .on_click(cx.listener(move |this, _, _, cx| {
                this.data.toggle_habit(&id, this.date);
                this.persist();
                cx.notify();
              })),
          );
        if streak > 0 {
          row = row.child(streak_chip(streak, p));
        }
        let menu_id = habit.id.clone();
        row = row.child(
          button(
            SharedString::from(format!("options-{menu_id}")),
            format!("Options for {}", habit.label),
            p,
          )
          .flex_none()
          .size(px(24.))
          .child(icon("dots", 16., p.soft))
          .on_click(cx.listener(move |this, event: &ClickEvent, window, cx| {
            this.menu_position = point(
              (event.position().x - px(120.))
                .max(px(8.))
                .min(window.viewport_size().width - px(140.)),
              (event.position().y + px(14.)).min(window.viewport_size().height - px(80.)),
            );
            this.menu = Some(menu_id.clone());
            cx.notify();
          })),
        );
      }
      list = list.child(row);
    }
    list.child(
      div()
        .flex()
        .items_center()
        .gap(px(10.))
        .h(px(34.))
        .text_color(p.faint)
        .child(icon("plus", 18., p.faint))
        .child(text_input(&self.habit_input, "Add habit", p, window, cx)),
    )
  }
  pub fn habit_menu(&self, cx: &mut Context<Self>) -> Div {
    let p = self.palette;
    let id = self.menu.clone().unwrap();
    let rename = id.clone();
    div()
      .absolute()
      .inset_0()
      .child(
        div()
          .id("menu-dismiss")
          .absolute()
          .inset_0()
          .on_click(cx.listener(|this, _, _, cx| {
            this.menu = None;
            cx.notify();
          })),
      )
      .child(
        div()
          .absolute()
          .left(self.menu_position.x)
          .top(self.menu_position.y)
          .w(px(132.))
          .p(px(4.))
          .bg(p.raised)
          .border_1()
          .border_color(p.line)
          .rounded(px(8.))
          .child(
            text_button("rename-habit", "Rename", p)
              .w_full()
              .justify_start()
              .on_click(cx.listener(move |this, _, window, cx| {
                this.begin_rename(rename.clone(), window, cx)
              })),
          )
          .child(
            text_button("delete-habit", "Delete", p)
              .w_full()
              .justify_start()
              .on_click(cx.listener(move |this, _, _, cx| {
                this.data.delete_habit(&id);
                this.menu = None;
                this.persist();
                cx.notify();
              })),
          ),
      )
  }
}
